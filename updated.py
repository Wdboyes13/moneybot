import subprocess
from git import Repo
import logging
import sys
import time

def update(repo_path):
    repo = Repo(repo_path)
    repo.git.pull('origin', 'Main')
    subprocess.run(["cargo", "build"])
    subprocess.run(["systemctl", "restart", "moneybot"])
    
def setup_logging():
    logging.basicConfig(
        level=logging.INFO,
        format='%(asctime)s - %(levelname)s - %(message)s',
        handlers=[
            logging.FileHandler('/var/log/moneybot_updater.log'),
            logging.StreamHandler(sys.stdout)
        ]
    )
    return logging.getLogger(__name__)

def monitor_updates(repo_path):
    logger = setup_logging()
    repo = Repo(repo_path)
    last_commit = repo.head.commit.hexsha

    logger.info(f"Monitoring {repo_path} for updates...")
    while True:
        try:
            repo.remotes.origin.fetch()
            local_commit = repo.head.commit.hexsha
            remote_commit = repo.commit('origin/' + repo.active_branch.name).hexsha

            if local_commit != remote_commit:
                logger.info(f"🚀 New updates available! Local: {local_commit[:8]}, Remote: {remote_commit[:8]}")
                update(repo_path=repo_path)
                last_commit = remote_commit
            else:
                logger.info(f"✅ Up-to-date at {local_commit[:8]}")
        except Exception as e:
            logger.error(f"Error checking for updates: {e}")
        time.sleep(30)

monitor_updates('.')