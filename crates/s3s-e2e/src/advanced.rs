use crate::case;

use s3s_test::tcx::TestContext;
use s3s_test::Result;
use s3s_test::TestFixture;
use s3s_test::TestSuite;

use std::ops::Not;
use std::sync::Arc;

use tracing::debug;

pub fn register(tcx: &mut TestContext) {
    case!(tcx, Advanced, STS, test_assume_role);
}

struct Advanced {
    sts: aws_sdk_sts::Client,
}

impl TestSuite for Advanced {
    async fn setup() -> Result<Self> {
        let sdk_conf = aws_config::from_env().load().await;
        let sts = aws_sdk_sts::Client::new(&sdk_conf);

        Ok(Self { sts })
    }
}

#[allow(clippy::upper_case_acronyms)]
struct STS {
    sts: aws_sdk_sts::Client,
}

impl TestFixture<Advanced> for STS {
    async fn setup(suite: Arc<Advanced>) -> Result<Self> {
        Ok(Self { sts: suite.sts.clone() })
    }
}

impl STS {
    async fn test_assume_role(self: Arc<Self>) -> Result<()> {
        let sts = &self.sts;

        let result = sts.assume_role().role_arn("example").role_session_name("test").send().await;

        let resp = result?;

        let credentials = resp.credentials().unwrap();
        assert!(credentials.access_key_id().is_empty().not(), "Expected non-empty access key ID");
        assert!(credentials.secret_access_key().is_empty().not(), "Expected non-empty secret access key");
        assert!(credentials.session_token().is_empty().not(), "Expected session token in the response");

        debug!(ak=?credentials.access_key_id());
        debug!(sk=?credentials.secret_access_key());
        debug!(st=?credentials.session_token());
        debug!(exp=?credentials.expiration());

        Ok(())
    }
}
