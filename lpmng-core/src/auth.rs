use biscuit_auth::{builder::*, error, Authorizer, Biscuit, KeyPair};

pub fn test() {
    let root = KeyPair::new();
    let public_key = root.public();

    // creating a first token
    let token1 = {
        let mut builder = Biscuit::builder(&root);

        // let's define some access rights
        builder.add_authority_fact("right(\"/a/file1.txt\", \"read\")")?;
        builder.add_authority_fact("right(\"/a/file1.txt\", \"write\")")?;
        builder.add_authority_fact("right(\"/a/file2.txt\", \"read\")")?;
        builder.add_authority_fact("right(\"/b/file3.txt\", \"write\")")?;

        // we can now create the token
        let biscuit = builder.build()?;
        println!("biscuit (authority): {}", biscuit.print());

        biscuit.to_vec()?
    };
}
