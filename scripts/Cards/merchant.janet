(def cost 3)
(def card-image "assets/image.png")

(def movement (std/plus 1))
(def attack (std/plus 1))

(def attack-strength 1)
(def defense 1)

(defn on-draw [game] nil)
(defn on-play [game] nil)
(defn on-discard [game] nil)
(defn on-ability [game] nil)
(defn on-turn_begin [game] (std/get-gold 3 (std/turn-player)))
(defn on-turn-end [game] nil)
