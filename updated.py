import subprocess
from git import Repo
import os
import time

def update(repo_path):
    repo = Repo(repo_path)
    repo.git.pull('origin', 'Main')
    subprocess.run(["cargo", "build"])
    subprocess.run(["systemctl", "restart", "moneybot"])

def monitor_updates(repo_path):
    repo = Repo(repo_path)
    last_commit = repo.head.commit.hexsha

    print(f"Monitoring {repo_path} for updates...")
    while True:
        try:
            repo.remotes.origin.fetch()
            local_commit = repo.head.commit.hexsha
            remote_commit = repo.commit('origin/' + repo.active_branch.name).hexsha

            if local_commit != remote_commit:
                print(f"🚀 New updates available! Local: {local_commit[:8]}, Remote: {remote_commit[:8]}")
                update(repo_path=repo_path)
                last_commit = remote_commit
            else:
                print(f"✅ Up-to-date at {local_commit[:8]}")
        except Exception as e:
            print(f"Error checking for updates: {e}")
        time.sleep(30)

monitor_updates('.')