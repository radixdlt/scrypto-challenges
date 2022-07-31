

export function promiseWithState(ogPromise: Promise) {
  var failed: bool = false;
  var succeeded: bool = false;
  var waiting: bool = true;

  var response = ogPromise.then(
    function(val) {
      succeeded = true;
      waiting = false;
      return val; 
    }, 

    function(err) {
      failed = true;
      waiting = false;
      throw err; 
    }
  );

  response.succeeded = function() { return succeeded; };
  response.failed = function() { return failed; };
  response.waiting = function() { return waiting; };

  return response;
}
