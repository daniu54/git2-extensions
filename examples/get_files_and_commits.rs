use git2_extensions::ExtendedRepository;

fn main() {
    let repo = &mut ExtendedRepository::new(".");

    repo.set_main_branch("main");

    let commits = repo.get_commits_of_current_branch();

    println!("Commits on current branch:");
    for commit in commits {
        println!("{:?}", commit);
    }
}
