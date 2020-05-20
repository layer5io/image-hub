<template>
    <v-app id="inspire">
        <h2>{{message}}</h2>
        <v-btn color="primary" @click="generateLoad">Generate Load</v-btn>
        <router-link :to="{name: 'Auth'}">
            <button id="myButton" class="foo bar">Go!</button>
        </router-link>
    </v-app>
</template>

<script>
import axios from 'axios';

const REQ_CNT = 10;

export default {
    created() {
        console.log('Component has been created!');
        let token = localStorage.getItem("token")
        axios.get("/api/hello", {
            headers: {
                Authorization: token
            }
        }).then((res) => {
            this.message = res.data 
            console.log(res)
        }).catch((res) => {
            console.log(res)
            this.message = res.response.data
        })
    },
    data: () => ({
        message: ""
    }),
  methods: {
    generateLoad: async function () {
        let pArr = []
        let token = localStorage.getItem("token")
        for (let i = 0; i < REQ_CNT; i++) {
            pArr.push(axios.get("/api/hello", {
            headers: {
                Authorization: token
            }
        }).catch(()=>{})
        )
        }
        await Promise.all(pArr)
        console.log("Done!")
    }
  }
}
</script>