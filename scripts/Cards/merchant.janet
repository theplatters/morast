(def cost 3)
(def card-image "../assets/image.jpg")

(def movement (std/plus 1))
(def attack (std/plus 1))

(def attack-strength 1)
(def defense 1)

(defn on-draw [context] nil)
(defn on-play [context] nil)
(defn on-discard [context] nil)
(defn on-ability [context] nil)
(defn on-turn_begin [context] (std/get-gold 3 (std/turn-player)))
(defn on-turn-end [context] nil)
