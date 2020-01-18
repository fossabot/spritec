const spritec = require('spritec_binding');
console.log(spritec.version());

// document.getElementById('spritec-version').innerText = `spritec version ${spritec.version()}`;

{
  let canvas = document.getElementById('spritec-canvas');
  let ctx = canvas.getContext('2d');
  ctx.imageSmoothingEnabled = false;
  ctx.scale(8, 8);

  const render = () => {
    let imageBuffer = new Uint8ClampedArray(spritec.render_sprite());
    let imageData = new ImageData(imageBuffer, 64);

    createImageBitmap(imageData).then((bitmap) => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      ctx.drawImage(bitmap, 0, 0);
      bitmap.close();
    });


    // window.requestAnimationFrame(render);
  }

  render();
  // window.requestAnimationFrame(render);
};
