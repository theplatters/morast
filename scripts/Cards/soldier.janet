(def cost 1)
(def card-image "assets/image2.png")

(def movement (std/plus 1))
(def attack (std/plus 1))

(def attack-strength 3)
(def defense 3)

(defn on-draw [game scheduler] nil)
(defn on-play [game scheduler] nil)
(defn on-discard [game scheduler] nil)
(defn on-ability [game scheduler] nil)
(defn on-turn-begin [game scheduler] nil)
(defn on-turn-end [game scheduler] nil)
