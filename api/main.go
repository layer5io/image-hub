package main

import (
	"encoding/base64"
	"encoding/json"
	"fmt"
	"io/ioutil"
	"net/http"
)

type User struct {
	Username string
	Password string
	Plan     string
}

var DB = map[string]User{}

func CORSMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.Method == http.MethodOptions {
			w.Header().Set("Access-Control-Allow-Origin", "*")
			w.Header().Set("Access-Control-Allow-Methods", "*")
			w.Header().Set("Access-Control-Allow-Headers", "*")
			w.Header().Set("Access-Control-Max-Age", "3600")
			w.WriteHeader(http.StatusNoContent)
			return
		}
		w.Header().Set("Access-Control-Allow-Origin", "*")
		next.ServeHTTP(w, r)
	})
}

func hello(w http.ResponseWriter, req *http.Request) {
	token := req.Header.Get("Authorization")
	w.Write([]byte("Hello, " + token + ", welcome to Istiocon 2021!"))
}

func auth(w http.ResponseWriter, req *http.Request) {
	defer req.Body.Close()
	data, err := ioutil.ReadAll(req.Body)
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		return
	}
	var mp map[string]string
	err = json.Unmarshal(data, &mp)
	fmt.Printf("1 %v", mp)
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}
	username := mp["username"]
	password := mp["password"]
	matchedUser, ok := DB[username]

	if !ok || matchedUser.Password != password {
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	token, err := json.Marshal(&map[string]string{
		"username": matchedUser.Username,
		"plan":     matchedUser.Plan,
	})

	fmt.Printf("3 %v", string(token))
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	json.NewEncoder(w).Encode(map[string]string{
		"token": base64.RawStdEncoding.EncodeToString(token),
	})
}

func upgrade(w http.ResponseWriter, req *http.Request) {
	defer req.Body.Close()
	data, err := ioutil.ReadAll(req.Body)
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		return
	}
	var mp map[string]string
	err = json.Unmarshal(data, &mp)
	fmt.Printf("1 %v", mp)
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}
	username := mp["username"]
	matchedUser, ok := DB[username]

	fmt.Printf("3 %v %v", matchedUser, ok)
	if !ok {
		w.WriteHeader(http.StatusBadRequest)
		return
	}

	matchedUser.Plan = mp["plan"]

	token, err := json.Marshal(&map[string]string{
		"username": matchedUser.Username,
		"plan":     matchedUser.Plan,
	})

	fmt.Printf("3 %v", string(token))
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}

	json.NewEncoder(w).Encode(map[string]string{
		"token": base64.RawStdEncoding.EncodeToString(token),
	})
}

func signup(w http.ResponseWriter, req *http.Request) {
	defer req.Body.Close()
	data, err := ioutil.ReadAll(req.Body)
	if err != nil {
		w.WriteHeader(http.StatusBadRequest)
		return
	}
	var mp map[string]string
	err = json.Unmarshal(data, &mp)
	fmt.Printf("1 %v", mp)
	if err != nil {
		w.WriteHeader(http.StatusInternalServerError)
		return
	}
	newUser := User{
		Username: mp["username"],
		Password: mp["password"],
		Plan:     mp["plan"],
	}
	DB[mp["username"]] = newUser
}

func main() {
	mux := http.NewServeMux()
	mux.Handle("/pull", CORSMiddleware(http.HandlerFunc(hello)))
	mux.Handle("/auth", CORSMiddleware(http.HandlerFunc(auth)))
	mux.Handle("/signup", CORSMiddleware(http.HandlerFunc(signup)))
	mux.Handle("/upgrade", CORSMiddleware(http.HandlerFunc(upgrade)))
	http.ListenAndServe(":9091", mux)
}
