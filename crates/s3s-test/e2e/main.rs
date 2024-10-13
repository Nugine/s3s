use s3s_test::Result;
use s3s_test::TestFixture;
use s3s_test::TestSuite;

use tracing::debug;

struct Basic {
    client: aws_sdk_s3::Client,
}

impl TestSuite for Basic {
    async fn setup() -> Result<Self> {
        let sdk_conf = aws_config::from_env().load().await;
        let s3_conf = aws_sdk_s3::config::Builder::from(&sdk_conf)
            .force_path_style(true) // FIXME: remove force_path_style
            .build();
        let client = aws_sdk_s3::Client::from_conf(s3_conf);
        Ok(Self { client })
    }
}

struct BasicOps {
    client: aws_sdk_s3::Client,
}

impl TestFixture<Basic> for BasicOps {
    async fn setup(suite: &Basic) -> Result<Self> {
        Ok(Self {
            client: suite.client.clone(),
        })
    }
}

impl BasicOps {
    async fn list_buckets(&self) -> Result<()> {
        let resp = self.client.list_buckets().send().await?;
        debug!(?resp);
        Ok(())
    }
}

fn main() {
    s3s_test::cli::main(|tcx| {
        macro_rules! case {
            ($s:ident, $x:ident, $c:ident) => {{
                let mut suite = tcx.suite::<$s>(stringify!($s));
                let mut fixture = suite.fixture::<$x>(stringify!($x));
                fixture.case(stringify!($c), $x::$c);
            }};
        }

        case!(Basic, BasicOps, list_buckets);
    })
}
