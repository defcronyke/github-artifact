(function () {
  const queryString = window.location.search;
  const urlParams = new URLSearchParams(queryString);

  if (!!urlParams.get('repo')) {
    window.location.replace('https://tinyurl.com/github-artifact' + queryString);
  }
})();
