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

const REQ_CNT = 500;
const BASE = "http://0.0.0.0:9091"
// const BASE = "/api"

export default {
    created() {
        console.log('Component has been created!');
        let token = localStorage.getItem("token")
        let userObj = JSON.parse(atob(token))
        axios.get(BASE + "/hello", {
            headers: {
                Authorization: token
            }
        }).then((res) => {
            this.message = `Hello ${userObj.username} (${userObj.plan})` 
            console.log(res)
        }).catch((res) => {
            console.log(res)
            if (res.response.status == 429) {
                this.message = "Limit Exceeded"
            } else {
                this.message = "Unauthorized"
            }
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
            pArr.push(axios.get(BASE + "/hello", {
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