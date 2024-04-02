use git2_extensions::Repository;

fn main() {
    let repo = &mut Repository::new(".");

    repo.set_main_branch("main");

    let commits = repo.get_commits_of_current_branch();

    println!("Commits on current branch:");
    for commit in commits {
        println!("{:#?}", commit);
    }
}
