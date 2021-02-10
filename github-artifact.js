(function() {
  const urlParams = new URLSearchParams(window.location.search);

  const repo = urlParams.get('repo');

  if (!!repo) {
    const repo_parts = repo.split('@');
    
    if (repo_parts.length <= 1) {
      return;
    }

    const auth_parts = repo_parts[0].split(':');
    
    if (auth_parts.length <= 1) {
      return;
    }

    const user = auth_parts[0];
    const token = auth_parts[1];

    const repo_path_parts = repo_parts[1].split('/');

    if (repo_path_parts.length <= 1) {
      return;
    }
      
    const repo_user = repo_path_parts[0];
    const repo_name = repo_path_parts[1];

    console.log('Getting github artifact: https://' + user + ':<redacted>@github.com/' + repo_user + '/' + repo_name);
  }
})();
