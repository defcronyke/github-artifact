(function() {
  const urlParams = window.location.search;

  if (!!urlParams) {
    window.location.replace('https://tinyurl.com/github-artifact' + urlParams);
  }
})();
