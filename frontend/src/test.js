export function testjs(){
    var video = document.getElementById('video');
      var videoSrc = '/live/test/hoge.m3u8';
      if (Hls.isSupported()) {
        var hls = new Hls();
        hls.loadSource(videoSrc);
        hls.attachMedia(video);
      }
}
