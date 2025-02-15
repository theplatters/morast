(def cost 1)
(def card-image "assets/image.png")
(def movement (std/plus 1))
(def attack (array/join (std/plus 1) (std/plus 2)))


(def attack-strength 3)
(def defense 1)

(defn on-draw [game] nil)
(defn on-play [game] nil)
(defn on-discard [game] nil)
(defn on-ability [game] nil)
(defn on-turn_begin [game] nil)
(defn on-turn-end [game] nil)
