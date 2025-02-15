(def cost 1)
(def card-image "assets/image.png")

(def movement @[])

(def attack (array/join (std/cross 1 ) (std/cross 2) (std/plus 2) (std/plus 1)))


(def attack-strength 3)
(def defense 2)

(defn on-draw [game] nil)
(defn on-play [game] nil)
(defn on-discard [game] nil)
(defn on-ability [game] nil)
(defn on-turn_begin [game] nil)
(defn on-turn-end [game] nil)
