(defn strip-extension [file] 
  (string "./" (string/replace ".janet"  "" file)))

(def base-dir (os/cwd))

(os/cd "game/scripts/Cards/Creatures")
(def creature-path (os/cwd))
(each file (os/dir creature-path) (import* (strip-extension file)) (printf "loaded %s" file))

(os/cd base-dir)
(os/cd "game/scripts/Cards/Spells")
(def spell-path (os/cwd))
(each file (os/dir spell-path) (import* (strip-extension file)) (printf "loaded %s" file))
(os/cd base-dir)
(import* "/game/scripts/cards")
(print "Loaded")
