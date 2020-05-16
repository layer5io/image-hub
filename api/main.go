package main

import (
	"encoding/json"
	"io/ioutil"
	"net/http"
)

func hello(w http.ResponseWriter, req *http.Request) {
	ck, err := req.Cookie("token")
	if err != nil {
		w.WriteHeader(http.StatusUnauthorized)
		return
	}
	token := ck.Value
	if token == "" {
		w.WriteHeader(http.StatusUnauthorized)
		return
	}
	w.Write([]byte("Hello, " + token + ", welcome to dockercon 2020!"))
}

func auth(w http.ResponseWriter, req *http.Request) {
	defer req.Body.Close()
	data, err := ioutil.ReadAll(req.Body)
	if err != nil {
		w.WriteHeader(http.StatusUnauthorized)
		http.SetCookie(w, &http.Cookie{
			Name:   "token",
			MaxAge: 0,
		})
		return
	}
	var mp map[string]string
	err = json.Unmarshal(data, &mp)
	if err != nil {
		w.WriteHeader(http.StatusUnauthorized)
		http.SetCookie(w, &http.Cookie{
			Name:   "token",
			MaxAge: 0,
		})
		return
	}
	username := mp["username"]
	if username == "" {
		w.WriteHeader(http.StatusUnauthorized)
		http.SetCookie(w, &http.Cookie{
			Name:   "token",
			MaxAge: 0,
		})
		return
	}
	http.SetCookie(w, &http.Cookie{
		Name:  "token",
		Value: username,
	})
}

func main() {
	http.HandleFunc("/hello", hello)
	http.HandleFunc("/auth", auth)
	http.ListenAndServe(":8080", nil)
}
