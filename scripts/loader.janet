(os/cd "scripts/Cards/")
(def card-path (os/cwd))
(import ./test)
(defn strip-extension [file] (string "./" (string/replace ".janet"  "" file)))
(each file (os/dir card-path) (import* (strip-extension file)))


(print (test/square-area 4))
(print "Loaded")
