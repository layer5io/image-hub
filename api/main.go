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

var DB = [4]User{User{
	Username: "Alice",
	Password: "alice",
	Plan:     "Enterprise",
}, User{
	Username: "Bob",
	Password: "bob",
	Plan:     "Team",
}, User{
	Username: "Eve",
	Password: "eve",
	Plan:     "Personal",
}}

func CORSMiddleware(next http.Handler) http.Handler {
	return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		// Our middleware logic goes here...
		// Set CORS headers for the preflight request
		if r.Method == http.MethodOptions {
			w.Header().Set("Access-Control-Allow-Origin", "*")
			w.Header().Set("Access-Control-Allow-Methods", "*")
			w.Header().Set("Access-Control-Allow-Headers", "*")
			w.Header().Set("Access-Control-Max-Age", "3600")
			w.WriteHeader(http.StatusNoContent)
			return
		}
		// Set CORS headers for the main request.
		w.Header().Set("Access-Control-Allow-Origin", "*")
		next.ServeHTTP(w, r)
	})
}

func hello(w http.ResponseWriter, req *http.Request) {
	token := req.Header.Get("Authorization")
	w.Write([]byte("Hello, " + token + ", welcome to dockercon 2020!"))
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
	var matchedUser User
	for _, item := range DB {
		if item.Username == username && item.Password == password {
			matchedUser.Username = username
			matchedUser.Plan = item.Plan
		}
	}
	fmt.Printf("2 %v", matchedUser)
	if matchedUser.Username == "" {
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

func main() {
	mux := http.NewServeMux()
	mux.Handle("/hello", CORSMiddleware(http.HandlerFunc(hello)))
	mux.Handle("/auth", CORSMiddleware(http.HandlerFunc(auth)))
	http.ListenAndServe(":9091", mux)
}
