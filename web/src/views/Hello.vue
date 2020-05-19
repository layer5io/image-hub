<template>
    <v-app id="inspire">
        <h2>{{message}}</h2>
    </v-app>
</template>

<script>
import axios from 'axios';
export default {
    created() {
        console.log('Component has been created!');
        let token = localStorage.getItem("token")
        axios.get("http://localhost:18000/hello", {
            headers: {
                Authorization: token
            }
        }).then((res) => {
            this.message = res.data 
            console.log(res)
        }).catch((res) => {
            console.log(res)
        })
    },
    data: () => ({
        message: ""
    }),
}
</script>