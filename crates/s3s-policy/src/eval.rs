use crate::model::WildcardOneOrMore;
use crate::model::{ActionRule, ResourceRule};
use crate::model::{Effect, Policy, Statement};
use crate::pattern::{PatternError, PatternSet};

use s3s::auth::S3AuthContext;
use s3s::path::S3Path;

pub struct SimpleEvaluator {
    rules: Vec<ParsedRule>,
}

#[derive(Debug, thiserror::Error)]
pub enum EvalError {
    #[error("Not supported")]
    NotSupported,

    #[error("Invalid pattern")]
    InvalidPattern,
}

struct ParsedRule {
    effect: Effect,
    action: Option<PatternSet>,
    not_action: Option<PatternSet>,
    resource: Option<PatternSet>,
    not_resource: Option<PatternSet>,
}

impl SimpleEvaluator {
    /// Create a new evaluator from a policy.
    ///
    /// # Errors
    /// Returns an error if the policy contains unsupported features.
    pub fn new(policy: &Policy) -> Result<Self, EvalError> {
        // TODO
        for statement in policy.statement.as_slice() {
            if statement.principal.is_some() {
                return Err(EvalError::NotSupported);
            }
            if statement.condition.is_some() {
                return Err(EvalError::NotSupported);
            }
        }

        let rules = policy
            .statement
            .as_slice()
            .iter()
            .map(Self::parse_rule)
            .collect::<Result<_, _>>()?;
        Ok(SimpleEvaluator { rules })
    }

    fn parse_rule(stmt: &Statement) -> Result<ParsedRule, EvalError> {
        let mut action = None;
        let mut not_action = None;
        let mut resource = None;
        let mut not_resource = None;

        let handle_error = |e: PatternError| match e {
            PatternError::InvalidPattern => EvalError::InvalidPattern,
        };

        let build = |w: &WildcardOneOrMore<String>| match w.as_slice() {
            Some(w) => PatternSet::new(w.iter().map(String::as_str)).map_err(handle_error),
            None => PatternSet::new(["*"]).map_err(handle_error),
        };

        match &stmt.action {
            ActionRule::Action(w) => action = Some(build(w)?),
            ActionRule::NotAction(w) => not_action = Some(build(w)?),
        }

        match &stmt.resource {
            ResourceRule::Resource(w) => resource = Some(build(w)?),
            ResourceRule::NotResource(w) => not_resource = Some(build(w)?),
        }

        Ok(ParsedRule {
            effect: stmt.effect.clone(),
            action,
            not_action,
            resource,
            not_resource,
        })
    }

    fn s3_op_to_action(op_name: &str) -> String {
        // FIXME: some op names are not exactly the same as the action names
        // https://docs.aws.amazon.com/AmazonS3/latest/userguide/using-with-s3-policy-actions.html

        // TODO: replace this with a static table
        if op_name == "ListBuckets" {
            return "s3:ListAllMyBuckets".to_owned();
        }

        format!("s3:{op_name}")
    }

    fn s3_path_to_resource(path: &S3Path) -> String {
        match path {
            S3Path::Root => "arn:aws:s3:::".to_owned(),
            S3Path::Bucket { bucket } => format!("arn:aws:s3:::{bucket}"),
            S3Path::Object { bucket, key } => format!("arn:aws:s3:::{bucket}/{key}"),
        }
    }

    /// Check if the request is allowed by the policy.
    ///
    /// # Errors
    ///
    pub fn check_access(&self, cx: &mut S3AuthContext<'_>) -> Result<Effect, EvalError> {
        Ok(self.check(cx.s3_op().name(), cx.s3_path()))
    }

    fn check(&self, op_name: &str, path: &S3Path) -> Effect {
        let action = Self::s3_op_to_action(op_name);
        let resource = Self::s3_path_to_resource(path);

        let mut allow_count = 0;
        for rule in &self.rules {
            let mut action_matched = false;
            if let Some(ps) = &rule.action {
                if ps.is_match(&action) {
                    action_matched = true;
                }
            } else if let Some(ps) = &rule.not_action {
                if !ps.is_match(&action) {
                    action_matched = true;
                }
            }

            let mut resource_matched = false;
            if let Some(ps) = &rule.resource {
                if ps.is_match(&resource) {
                    resource_matched = true;
                }
            } else if let Some(ps) = &rule.not_resource {
                if !ps.is_match(&resource) {
                    resource_matched = true;
                }
            }

            let matched = action_matched && resource_matched;

            if matched {
                match rule.effect {
                    Effect::Allow => {
                        allow_count += 1;
                    }
                    Effect::Deny => {
                        return Effect::Deny;
                    }
                }
            }
        }

        if allow_count > 0 {
            return Effect::Allow;
        }

        Effect::Deny
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let json = crate::tests::example1_json();
        let mut policy: Policy = serde_json::from_str(json).unwrap();

        // TODO
        for stmt in policy.statement.as_mut_slice() {
            stmt.principal = None;
            stmt.condition = None;
        }

        let evaluator = SimpleEvaluator::new(&policy).unwrap();

        {
            let ans = evaluator.check("ListBuckets", &S3Path::root());
            assert_eq!(ans, Effect::Allow);
        }
        {
            let ans = evaluator.check("ListObjectsV2", &S3Path::bucket("confidential-data"));
            assert_eq!(ans, Effect::Allow);

            let ans = evaluator.check("GetObject", &S3Path::object("confidential-data", "file.txt"));
            assert_eq!(ans, Effect::Allow);

            let ans = evaluator.check("PutObject", &S3Path::bucket("confidential-data"));
            assert_eq!(ans, Effect::Deny);
        }
        {
            let ans = evaluator.check("ListObjectsV2", &S3Path::bucket("another-bucket"));
            assert_eq!(ans, Effect::Deny);

            let ans = evaluator.check("GetObject", &S3Path::object("another-bucket", "file.txt"));
            assert_eq!(ans, Effect::Deny);
        }
    }
}
