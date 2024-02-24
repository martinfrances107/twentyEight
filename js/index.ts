import('../pkg')
  .then(twenty_eight => {
    log("wasm is imported");

    const perf = document.getElementById('perf')

    if (perf != null) {
      perf.innerHTML = 'Render Time: ...Calculating'


      twenty_eight.Game.default()
        .then((game) => {

        }).catch(() => { log('unable to construct Renderer.') })
    }




  })


