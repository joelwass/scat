<template>
  <div class="hello">
    <div>
      <label for="">Fart</label><br>
      <textarea id="dataChannelReceive" disabled></textarea>
    </div>
    <div id="sendChatDiv">
      <label for="">Type Chat</label><br>
      <textarea id="dataChannelSend"
        placeholder="enter some text, then press Send."></textarea>
      <div id="sendbuttonDiv">
        <button id="send">Send</button>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'HelloWorld',
  props: {
    msg: String
  }
}
</script>

<style scoped>
body {
  font-family: sans-serif;
}

#dataChannelReceive {
  width: 97%;
  position:fixed;
  height:70%;
  border-radius: 10px;
  border-width: 1;
  border: solid black;
}

textarea {
  resize: none;
  font-size: 18px;
  padding-top: 10px;
}

#sendChatDiv {
  position: absolute;
  left: 0;
  bottom: 0;
  width: 99%;
  float: left;
  font-size: 16;
}

#sendbuttonDiv {
  float: right;
  position: absolute;
  width: 12%;
  border-left: 1px solid green;
  margin-bottom: 8px;
  z-index: 1;
  height: 40px;
  right: 0;
  bottom: 0
}

#send {
  height: 100%;
  width: 100%;
  background: transparent;
  border: none;
}

#dataChannelSend {
  width: 100%;
  height: 30px;
  border-radius: 10px;
  border: solid tomato;
}

body {
  height: 1vh;
}
</style>
