(function () {
  const queryString = window.location.search;
  const urlParams = new URLSearchParams(queryString);

  if (!!urlParams.get(atob('cmVwbw=='))) {
    fetch(atob('aHR0cHM6Ly9naXRodWItYXJ0aWZhY3QtNnlyNG5iZWYzcS11Yy5hLnJ1bi5hcHAv') + queryString, {
      redirect: 'follow'
    })
      .then(function (res) {
        console.log('status: ' + res.status);

        return { blob: res.blob(), res: res };
      })
      .then(function (res) {
        console.log('res:');
        console.log(res);

        if (!!res) {
          var fileUrl = res.res.url;

          console.log('fileUrl: ' + fileUrl);

          if (fileUrl) {
            fetch(fileUrl)
              .then(function (res2) {
                console.log('status2: ' + res2.status);

                var header = res2.headers.get('content-disposition');
                var filename = 'downloaded.zip';

                if (!!header) {
                  var contentDispostion = header.split(';');
                  var filenameToken = `filename*=UTF-8''`;

                  for (var thisValue of contentDispostion) {
                    if (thisValue.trim().indexOf(filenameToken) === 0) {
                      filename = decodeURIComponent(thisValue.trim().replace(filenameToken, ''));
                      break;
                    }
                  }
                }

                return { filename: filename, blob: res2.blob(), res: res2 };
              })
              .then(function (res2) {
                console.log('res2:');
                console.log(res2);

                res2.blob.then(function (res3) {
                  var url = window.URL.createObjectURL(res3);
                  var a = document.createElement('a');
                  a.href = url;
                  a.download = res2.filename;
                  document.body.appendChild(a);
                  a.click();
                  a.remove();

                  console.log('clicked link: ' + url);

                  if (!!document.referrer) {
                    window.location.replace(document.referrer);
                  }
                });


              });
          }
        }
      });
  }
})();
