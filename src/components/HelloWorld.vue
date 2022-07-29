<template>
  <div class="hello">
    <h1>{{ msg }}</h1>
    <p>
      For a guide and recipes on how to configure / customize this project,<br>
      check out the
      <a href="https://cli.vuejs.org" target="_blank" rel="noopener">vue-cli documentation</a>.
    </p>
    <h3>Installed CLI Plugins</h3>
    <ul>
      <li><input v-model="m3u8_file" ></li>
    </ul>
    <ul>
      <li><input v-model="output" ></li>
    </ul>
    <ul>
      <li><button @click="uploadfile">上传</button></li>
    </ul>
  </div>
</template>

<script>
import { invoke } from '@tauri-apps/api'
// import { listen } from '@tauri-apps/api/event'

export default {
  name: 'HelloWorld',
  props: {
    msg: String
  },
  data() {
    return {
      m3u8_file:"http://1257120875.vod2.myqcloud.com/0ef121cdvodtransgzp1257120875/3055695e5285890780828799271/v.f230.m3u8",
      output:"11.mp4"
    }
  },
  methods: {
    uploadfile() {
      console.log("m3u8_file", this.m3u8_file, "output", this.output);
      invoke('downFile', { filePath: this.m3u8_file, output: this.output }).then((response) => console.log(response))
    }
  },
}

// listen('downing', (event) => {
//   console.log("listen download...");
//   console.log(event);
// });

</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
h3 {
  margin: 40px 0 0;
}
ul {
  list-style-type: none;
  padding: 0;
}
li {
  display: inline-block;
  margin: 0 10px;
}
a {
  color: #42b983;
}
</style>
