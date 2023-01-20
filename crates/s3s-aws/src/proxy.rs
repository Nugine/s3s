use aws_sdk_s3::Client;

pub struct Proxy(Client);

impl From<Client> for Proxy {
    fn from(val: Client) -> Self {
        Self(val)
    }
}
