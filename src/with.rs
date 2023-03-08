use inquire::MultiSelect;

use crate::coauthor_repo::CoauthorRepo;

pub(crate) fn handle(coauthor_repo: &Box<dyn CoauthorRepo>, coauthor_keys: &Vec<String>) {
    match coauthor_keys.len() {
        0 => {
            let coauthors = coauthor_repo.list();
            let result = MultiSelect::new("Select active co-author(s):", coauthors).prompt();

            match result {
                Ok(selected) => {
                    coauthor_repo.deactivate_all();

                    selected.clone().into_iter().for_each(|coauthor| {
                        coauthor_repo.activate(&coauthor);
                    });

                    if selected.is_empty() {
                        println!("Going solo!")
                    }
                }
                Err(_) => println!("failed to select co-author(s)"),
            }
        }
        _ => {
            coauthor_repo.deactivate_all();

            let coauthors: Vec<String> = coauthor_keys
                .into_iter()
                .map(|key| {
                    let coauthor = coauthor_repo.get(&key);
                    coauthor_repo.activate(&coauthor);
                    return coauthor;
                })
                .collect();

            println!("{}", coauthors.join("\n"));
        }
    }
}
