
module.exports = function(app){ return {
  post: function(path, processor){
    app.post(path, async (req,res) => {
      try {
        res.send(await processor(req));
      }catch(ex){
        console.log(ex);
        res.send({error: ex.message || "unknown error"});
      }
    });
  },
  
}}