
export function testjs(){


    document.getElementById("kaka").textContent = "OK";
    alert("tezt");
    var video = document.getElementById('test');
      var videoSrc = '/live/test/hoge.m3u8';
      if (Hls.isSupported()) {
        var hls = new Hls();
        hls.loadSource(videoSrc);
        hls.attachMedia(video);
      }
    //   else if (video.canPlayType('')) {
    //     video.src = videoSrc;
    //   }
}
