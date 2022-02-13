pub trait OciRegistryAuth {
    fn auth();
}

pub trait OciRegistry {
    fn push();
    fn pull();
}

pub struct OciRegistryAuthAnonymous;

pub struct OciRegistryAuthUserPass {
    username: String,
    password: String,
}

pub struct DefaultOciRegistry {
    pub url: String,
    pub auth: Box<dyn OciRegistryAuth>,
}

impl OciRegistryAuth for OciRegistryAuthAnonymous {
    fn auth() {
        todo!()
    }
}

impl OciRegistryAuth for OciRegistryAuthUserPass {
    fn auth() {
        todo!()
    }
}

impl OciRegistry for DefaultOciRegistry {
    fn push() {
        todo!()
    }

    fn pull() {
        todo!()
    }
}