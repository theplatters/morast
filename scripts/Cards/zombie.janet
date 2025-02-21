(def cost 1)
(def card-image "assets/image3.png")

(def movement (std/plus 1))

(def attack (array/join (std/cross 1) (std/plus 1)))


(def attack-strength 2)
(def defense 1)

(defn on-draw [game scheduler] nil)
(defn on-play [game scheduler] (std/apply-effect game scheduler 'weakening  2 (map (fn [x] (map + x (std/current-index game))) (std/plus 1))))
(defn on-discard [game scheduler] nil)
(defn on-ability [game scheduler] nil)
(defn on-turn-begin [game scheduler] nil)
(defn on-turn-end [game scheduler] nil)
