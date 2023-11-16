use mongodb::{bson::doc , bson::Bson , Collection};

async fn add_verb_forms(collection : &Collection , verb : &str ,forms:Vec<&str>) -> mongo::error::Result<()> {
    let form_bson: Vec<Bson> = forms.into_iter().map(Bson::from).collection();
    let verb_forms = doc! {
        "verb": verb,
        "forms": form_bson,
    }

    collection.insert_one(verb_forms , None).await?;
    Ok(())
}
