(def cost 3)
(def card-image "assets/image.png")

(def movement (std/plus 1))
(def attack (std/plus 1))

(def attack-strength 1)
(def defense 1)

(defn on-draw [game scheduler] nil)
(defn on-play [game scheduler] nil)
(defn on-discard [game scheduler] nil)
(defn on-ability [game scheduler] nil)
(defn on-turn-begin [game sced] (if (= (std/owner game) (std/turn-player game)) (std/get-gold game 3 (std/owner game))) (print (std/turn-player game)))
(defn on-turn-end [game scheduler] nil)
