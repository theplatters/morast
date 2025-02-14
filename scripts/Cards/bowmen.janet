(def cost 1)
(def card-image "../assets/image.png")
(def movement (std/plus 1))
(def attack (array/join (std/plus 1) (std/plus 2)))


(def attack-strength 3)
(def defense 1)

(defn on-draw [context] nil)
(defn on-play [context] nil)
(defn on-discard [context] nil)
(defn on-ability [context] nil)
(defn on-turn_begin [context] nil)
(defn on-turn-end [context] nil)
