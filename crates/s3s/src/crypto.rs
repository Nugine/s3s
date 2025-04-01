pub trait Checksum {
    type Output: AsRef<[u8]>;

    #[must_use]
    fn new() -> Self;

    fn update(&mut self, data: &[u8]);

    #[must_use]
    fn finalize(self) -> Self::Output;

    #[must_use]
    fn checksum(data: &[u8]) -> Self::Output
    where
        Self: Sized,
    {
        let mut hasher = Self::new();
        hasher.update(data);
        hasher.finalize()
    }
}

#[derive(Default)]
pub struct Crc32(crc32fast::Hasher);

impl Checksum for Crc32 {
    type Output = [u8; 4];

    fn new() -> Self {
        Self::default()
    }

    fn update(&mut self, data: &[u8]) {
        self.0.update(data);
    }

    fn finalize(self) -> Self::Output {
        self.0.finalize().to_be_bytes()
    }
}

#[derive(Default)]
pub struct Crc32c(u32);

impl Checksum for Crc32c {
    type Output = [u8; 4];

    fn new() -> Self {
        Self::default()
    }

    fn update(&mut self, data: &[u8]) {
        self.0 = crc32c::crc32c_append(self.0, data);
    }

    fn finalize(self) -> Self::Output {
        self.0.to_be_bytes()
    }
}

#[derive(Default)]
pub struct Crc64Nvme(crc64fast_nvme::Digest);

impl Checksum for Crc64Nvme {
    type Output = [u8; 8];

    fn new() -> Self {
        Self::default()
    }

    fn update(&mut self, data: &[u8]) {
        self.0.write(data);
    }

    fn finalize(self) -> Self::Output {
        self.0.sum64().to_be_bytes()
    }
}

#[derive(Default)]
pub struct Sha1(sha1::Sha1);

impl Checksum for Sha1 {
    type Output = [u8; 20];

    fn new() -> Self {
        Self::default()
    }

    fn update(&mut self, data: &[u8]) {
        use sha1::Digest as _;
        self.0.update(data);
    }

    fn finalize(self) -> Self::Output {
        use sha1::Digest as _;
        self.0.finalize().into()
    }
}

#[derive(Default)]
pub struct Sha256(sha2::Sha256);

impl Checksum for Sha256 {
    type Output = [u8; 32];

    fn new() -> Self {
        Self::default()
    }

    fn update(&mut self, data: &[u8]) {
        use sha2::Digest as _;
        self.0.update(data);
    }

    fn finalize(self) -> Self::Output {
        use sha2::Digest as _;
        self.0.finalize().into()
    }
}

#[derive(Default)]
pub struct Md5(md5::Md5);

impl Checksum for Md5 {
    type Output = [u8; 16];

    fn new() -> Self {
        Self::default()
    }

    fn update(&mut self, data: &[u8]) {
        use md5::Digest as _;
        self.0.update(data);
    }

    fn finalize(self) -> Self::Output {
        use md5::Digest as _;
        self.0.finalize().into()
    }
}
