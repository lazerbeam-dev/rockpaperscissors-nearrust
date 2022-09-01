<template>
  <div>
    <button class="option" @click="this.test()">
      <img class="optionImg" src="https://cdn-icons-png.flaticon.com/512/2041/2041965.png" alt="rock">
    </button>
    <button class="option" @click="this.test()">
      <img class="optionImg" src="https://cdn-icons-png.flaticon.com/512/3616/3616639.png" alt="paper">
    </button>
    <button class="option" @click="this.test()">
      <img class="optionImg" src="https://cdn-icons-png.flaticon.com/512/737/737812.png" alt="scissors">
    </button>
  </div>
</template>

<script>

export default {
  name: 'HelloWorld',
  props: {
    msg: String
  },
  methods: { test(){
      console.log('test')
    }
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
.optionImg{
  max-width: 20%;
  max-height: 20%;
}
.option{
  display: inline-block;
  max-width: 30%;
}
</style>
