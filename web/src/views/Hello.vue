<template>
    <v-app id="inspire">
        <h2>{{message}}</h2>
        <v-btn color="primary" @click="generateLoad">Generate Load</v-btn>
    </v-app>
</template>

<script>
import axios from 'axios';

const REQ_CNT = 500;

export default {
    created() {
        console.log('Component has been created!');
        let token = localStorage.getItem("token")
        if (token == null) {
            this.message = "Unauthorized"
            return
        }
        let userObj = JSON.parse(atob(token))
        axios.get(this.$BASE_URL + "/hello", {
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
            pArr.push(axios.get(this.$BASE_URL + "/hello", {
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