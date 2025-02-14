(def cost 1)
(def card-image "../assets/image.jpg")

(def movement (std/plus 1))
(def attack (std/plus 1))

(def attack-strength 3)
(def defense 3)

(defn on-draw [context] nil)
(defn on-play [context] nil)
(defn on-discard [context] nil)
(defn on-ability [context] nil)
(defn on-turn_begin [context] nil)
(defn on-turn-end [context] nil)
