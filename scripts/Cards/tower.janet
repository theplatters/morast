(def cost 1)
(def card-image "assets/image3.png")

(def movement @[])

(def attack (array/join (std/cross 1 ) (std/cross 2) (std/plus 2) (std/plus 1)))


(def attack-strength 3)
(def defense 2)

(defn on-draw [game scheduler] nil)
(defn on-play [game scheduler] nil)
(defn on-discard [game scheduler] nil)
(defn on-ability [game scheduler] nil)
(defn on-turn-begin [game scheduler] nil)
(defn on-turn-end [game scheduler] nil)
