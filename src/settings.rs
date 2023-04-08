#[derive(asr::Settings)]
pub struct Settings {
    #[default = false]
    /// Load/Autosave Remover
    pub load_removal: bool,

    #[default = false]
    /// Throne Main Story Complete (End Single Story)
    pub throne_story_complete: bool,
    #[default = false]
    /// Throne joins the party
    pub throne_joins: bool,
    #[default = false]
    /// Throne 1 - On the Run
    pub throne_10: bool,
    #[default = false]
    /// Throne 1 - A Locked Door
    pub throne_20: bool,
    #[default = false]
    /// Throne 1 - A Traitor Among Us
    pub throne_30: bool,
    #[default = false]
    /// Throne 1 - Silencing the Guard
    pub throne_40: bool,
    #[default = false]
    /// Throne 1 - Meaningless Bloodshed
    pub throne_50: bool,
    #[default = false]
    /// Throne 1 - Enemies Lying in Wait
    pub throne_60: bool,
    #[default = false]
    /// Throne 1 - To the World Above
    pub throne_70: bool,
    #[default = false]
    /// Throne 1 - The Blacksnakes
    pub throne_80: bool,
    #[default = false]
    /// Throne 1 - Father
    pub throne_90: bool,
    #[default = false]
    /// Throne 1 - Mother
    pub throne_100: bool,
    #[default = false]
    /// Throne 1 - My Dream
    pub throne_110: bool,
    #[default = false]
    /// Throne 1 - Scaracci the Traitor
    pub throne_120: bool,
    #[default = false]
    /// Throne 1 - Diamante's Estate
    pub throne_150: bool,
    #[default = false]
    /// Throne 1 - Successful Infiltration
    pub throne_160: bool,
    #[default = false]
    /// Throne 1 - Selecting a Successor
    pub throne_180: bool,
    #[default = false]
    /// Throne 1 - Freedom
    pub throne_190: bool,
    #[default = false]
    /// Throne 1 - Throné's Resolve
    pub throne_500: bool,
    #[default = false]
    /// Throne 2A - Pursuing Mother
    pub throne_510: bool,
    #[default = false]
    /// Throne 2A - Feigning Ignorance
    pub throne_520: bool,
    #[default = false]
    /// Throne 2A - A Horse
    pub throne_530: bool,
    #[default = false]
    /// Throne 2A - Information on Mother
    pub throne_540: bool,
    #[default = false]
    /// Throne 2A - Followed
    pub throne_550: bool,
    #[default = false]
    /// Throne 2A - The Pursuer's Face
    pub throne_560: bool,
    #[default = false]
    /// Throne 2A - The Masked Boy
    pub throne_570: bool,
    #[default = false]
    /// Throne 2A - To the Old Foundry
    pub throne_580: bool,
    #[default = false]
    /// Throne 2A - The Slaver
    pub throne_590: bool,
    #[default = false]
    /// Throne 2A - Death's Table
    pub throne_610: bool,
    #[default = false]
    /// Throne 2A - The Boy's True Face
    pub throne_1000: bool,
    #[default = false]
    /// Throne 2B - Pursuing Father
    pub throne_1010: bool,
    #[default = false]
    /// Throne 2B - Trouble at the Tavern
    pub throne_1020: bool,
    #[default = false]
    /// Throne 2B - Father Appears
    pub throne_1030: bool,
    #[default = false]
    /// Throne 2B - Visiting the Snowhares
    pub throne_1040: bool,
    #[default = false]
    /// Throne 2B - Hesitation
    pub throne_1050: bool,
    #[default = false]
    /// Throne 2B - First Job
    pub throne_1060: bool,
    #[default = false]
    /// Throne 2B - Tainted with the Stench of Blood
    pub throne_1070: bool,
    #[default = false]
    /// Throne 2B - At the Snowhares' Den
    pub throne_1080: bool,
    #[default = false]
    /// Throne 2B - Bergomi
    pub throne_1090: bool,
    #[default = false]
    /// Throne 2B - True Intentions
    pub throne_1100: bool,
    #[default = false]
    /// Throne 2B - If I Am to Be Free
    pub throne_1500: bool,
    #[default = false]
    /// Throne 3A - Looking the Part
    pub throne_1510: bool,
    #[default = false]
    /// Throne 3A - A Young Thief
    pub throne_1520: bool,
    #[default = false]
    /// Throne 3A - The Bell Chimes
    pub throne_1530: bool,
    #[default = false]
    /// Throne 3A - A True Snake
    pub throne_1540: bool,
    #[default = false]
    /// Throne 3A - Subordination
    pub throne_1550: bool,
    #[default = false]
    /// Throne 3A - Mother Must Die
    pub throne_1560: bool,
    #[default = false]
    /// Throne 3A - Key to the Door
    pub throne_1570: bool,
    #[default = false]
    /// Throne 3A - Clash with Mother
    pub throne_1590: bool,
    #[default = false]
    /// Throne 3A - The New Leader
    pub throne_1600: bool,
    #[default = false]
    /// Throne 3A - Mother's Final Moments
    pub throne_1610: bool,
    #[default = false]
    /// Throne 3A - Daughter of Snakes
    pub throne_2000: bool,
    #[default = false]
    /// Throne 3B - Where Father Awaits
    pub throne_2010: bool,
    #[default = false]
    /// Throne 3B - Destiny
    pub throne_2020: bool,
    #[default = false]
    /// Throne 3B - Marietta
    pub throne_2030: bool,
    #[default = false]
    /// Throne 3B - The Abandoned Church
    pub throne_2040: bool,
    #[default = false]
    /// Throne 3B - That Which Cannot Be Stolen
    pub throne_2050: bool,
    #[default = false]
    /// Throne 3B - Baby
    pub throne_2060: bool,
    #[default = false]
    /// Throne 3B - Clash with Father
    pub throne_2070: bool,
    #[default = false]
    /// Throne 3B - A Father and Daughter
    pub throne_2080: bool,
    #[default = false]
    /// Throne 3B - Dad
    pub throne_2090: bool,
    #[default = false]
    /// Throne 3B - A Heart Unfulfilled
    pub throne_2500: bool,
    #[default = false]
    /// Throne 4 - Orphaned Snakes
    pub throne_2510: bool,
    #[default = false]
    /// Throne 4 - The Scent of Cigarettes
    pub throne_2520: bool,
    #[default = false]
    /// Throne 4 - The Blacksnakes' Cemetery
    pub throne_2530: bool,
    #[default = false]
    /// Throne 4 - Engraved on the Tomb
    pub throne_2540: bool,
    #[default = false]
    /// Throne 4 - Where It All Began
    pub throne_2550: bool,
    #[default = false]
    /// Throne 4 - A Door Unlocked
    pub throne_2560: bool,
    #[default = false]
    /// Throne 4 - A Ropeway Home
    pub throne_2580: bool,
    #[default = false]
    /// Throne 4 - In the Gondola
    pub throne_2600: bool,
    #[default = false]
    /// Throne 4 - To Lostseed
    pub throne_2610: bool,
    #[default = false]
    /// Throne 4 - A Baby's Cries
    pub throne_2620: bool,
    #[default = false]
    /// Throne 4 - An Old Castle
    pub throne_2630: bool,
    #[default = false]
    /// Throne 4 - Claude, the Father
    pub throne_2640: bool,
    #[default = false]
    /// Throne 4 - His Greatest Masterpiece
    pub throne_2650: bool,
    #[default = false]
    /// Throne 4 - A Collar Removed
    pub throne_2660: bool,
    #[default = false]
    /// Throne 4 - Throné
    pub throne_2670: bool,
    #[default = false]
    /// Throne 4 - The Stench of Blood
    pub throne_3000: bool,

    #[default = false]
    /// Temenos Main Story Complete (End Single Story)
    pub temenos_story_complete: bool,
    #[default = false]
    /// Temenos joins the party
    pub temenos_joins: bool,
    #[default = false]
    /// Temenos 1 - The Eight Gods
    pub temenos_10: bool,
    #[default = false]
    /// Temenos 1 - The Wicked God, Vide
    pub temenos_20: bool,
    #[default = false]
    /// Temenos 1 - Temenos and the Children
    pub temenos_30: bool,
    #[default = false]
    /// Temenos 1 - Temenos and the Pontiff
    pub temenos_40: bool,
    #[default = false]
    /// Temenos 1 - Crick, Newly Anointed Knight
    pub temenos_60: bool,
    #[default = false]
    /// Temenos 1 - Godsblade
    pub temenos_90: bool,
    #[default = false]
    /// Temenos 1 - The Locked Cathedral
    pub temenos_100: bool,
    #[default = false]
    /// Temenos 1 - An Unconventional Inquisitor
    pub temenos_110: bool,
    #[default = false]
    /// Temenos 1 - The Pontiff's Death
    pub temenos_120: bool,
    #[default = false]
    /// Temenos 1 - Suspected Foul Play
    pub temenos_130: bool,
    #[default = false]
    /// Temenos 1 - The Sacred Guard Arrives
    pub temenos_140: bool,
    #[default = false]
    /// Temenos 1 - The Pontiff's Funeral
    pub temenos_150: bool,
    #[default = false]
    /// Temenos 1 - A Note Between the Pages
    pub temenos_160: bool,
    #[default = false]
    /// Temenos 1 - Reassigned
    pub temenos_170: bool,
    #[default = false]
    /// Temenos 1 - In Search of Truth
    pub temenos_500: bool,
    #[default = false]
    /// Temenos 2 - The Second Victim
    pub temenos_510: bool,
    #[default = false]
    /// Temenos 2 - Lucian the Theologian
    pub temenos_520: bool,
    #[default = false]
    /// Temenos 2 - Disturbance in Canalbrine
    pub temenos_530: bool,
    #[default = false]
    /// Temenos 2 - Reunited with Crick
    pub temenos_540: bool,
    #[default = false]
    /// Temenos 2 - Lucian the Culprit?
    pub temenos_550: bool,
    #[default = false]
    /// Temenos 2 - A Forceful Entry
    pub temenos_560: bool,
    #[default = false]
    /// Temenos 2 - The Third Victim
    pub temenos_570: bool,
    #[default = false]
    /// Temenos 2 - Captain Kaldena
    pub temenos_580: bool,
    #[default = false]
    /// Temenos 2 - The Search for Evidence
    pub temenos_590: bool,
    #[default = false]
    /// Temenos 2 - Clue: Note
    pub temenos_600: bool,
    #[default = false]
    /// Temenos 2 - Clue: Book
    pub temenos_610: bool,
    #[default = false]
    /// Temenos 2 - Clue: Memo
    pub temenos_620: bool,
    #[default = false]
    /// Temenos 2 - Dancer at the Tavern
    pub temenos_630: bool,
    #[default = false]
    /// Temenos 2 - The Killer Is Among Us
    pub temenos_640: bool,
    #[default = false]
    /// Temenos 2 - The Culprit Is You!
    pub temenos_650: bool,
    #[default = false]
    /// Temenos 2 - The Fleeing Vados
    pub temenos_660: bool,
    #[default = false]
    /// Temenos 2 - Time for Answers
    pub temenos_670: bool,
    #[default = false]
    /// Temenos 2 - The Sacred Guard Interferes
    pub temenos_680: bool,
    #[default = false]
    /// Temenos 2 - Kaldena and Crick
    pub temenos_690: bool,
    #[default = false]
    /// Temenos 2 - Ort the Sanctum Knight
    pub temenos_700: bool,
    #[default = false]
    /// Temenos 2 - Two Paths
    pub temenos_1000: bool,
    #[default = false]
    /// Temenos 3A - Before the Inquisition
    pub temenos_1010: bool,
    #[default = false]
    /// Temenos 3A - Mindt's Letter
    pub temenos_1020: bool,
    #[default = false]
    /// Temenos 3A - To the Sacred Guard Headquarters
    pub temenos_1030: bool,
    #[default = false]
    /// Temenos 3A - A Shocking Reunion
    pub temenos_1040: bool,
    #[default = false]
    /// Temenos 3A - No Record of Arrival
    pub temenos_1050: bool,
    #[default = false]
    /// Temenos 3A - Mysterious Meddler
    pub temenos_1060: bool,
    #[default = false]
    /// Temenos 3A - Vados the Corpse
    pub temenos_1070: bool,
    #[default = false]
    /// Temenos 3A - Bring the Truth to Light
    pub temenos_1080: bool,
    #[default = false]
    /// Temenos 3A - In Search of Evidence, Alone
    pub temenos_1100: bool,
    #[default = false]
    /// Temenos 3A - Crick in the Archives
    pub temenos_1110: bool,
    #[default = false]
    /// Temenos 3A - Temenos at the Inn
    pub temenos_1120: bool,
    #[default = false]
    /// Temenos 3A - An Omen
    pub temenos_1130: bool,
    #[default = false]
    /// Temenos 3A - Crick's Death
    pub temenos_1140: bool,
    #[default = false]
    /// Temenos 3A - Crick's Clue: Words in the Shelf
    pub temenos_1170: bool,
    #[default = false]
    /// Temenos 3A - Crick's Clue: Mysterious Mechanism
    pub temenos_1190: bool,
    #[default = false]
    /// Temenos 3A - A Hidden Passageway
    pub temenos_1200: bool,
    #[default = false]
    /// Temenos 3A - The Book of Night
    pub temenos_1210: bool,
    #[default = false]
    /// Temenos 3A - Crick's Murderer
    pub temenos_1220: bool,
    #[default = false]
    /// Temenos 3A - The Mastermind
    pub temenos_1230: bool,
    #[default = false]
    /// Temenos 3A - Kaldena's Plans
    pub temenos_1240: bool,
    #[default = false]
    /// Temenos 3A - Temenos's Resolve
    pub temenos_1250: bool,
    #[default = false]
    /// Temenos 3A - To End Kaldena's Treachery
    pub temenos_1500: bool,
    #[default = false]
    /// Temenos 3B - The Fellsun Ruins
    pub temenos_1510: bool,
    #[default = false]
    /// Temenos 3B - Renouncers of the Sacred Flame
    pub temenos_1530: bool,
    #[default = false]
    /// Temenos 3B - Roi
    pub temenos_1540: bool,
    #[default = false]
    /// Temenos 3B - The Investigation Begins
    pub temenos_1550: bool,
    #[default = false]
    /// Temenos 3B - Guiding a Troubled Traveler
    pub temenos_1560: bool,
    #[default = false]
    /// Temenos 3B - Reiza's Purpose
    pub temenos_1580: bool,
    #[default = false]
    /// Temenos 3B - The Kal Ruins
    pub temenos_1590: bool,
    #[default = false]
    /// Temenos 3B - Mysterious Mural
    pub temenos_1600: bool,
    #[default = false]
    /// Temenos 3B - Horror in the Ruins
    pub temenos_1610: bool,
    #[default = false]
    /// Temenos 3B - Rite of the Night
    pub temenos_1620: bool,
    #[default = false]
    /// Temenos 3B - Sins of the Moonshade Order
    pub temenos_1630: bool,
    #[default = false]
    /// Temenos 3B - Uncovering the Truth
    pub temenos_2000: bool,
    #[default = false]
    /// Temenos 4 - Memories of a Massacre
    pub temenos_2010: bool,
    #[default = false]
    /// Temenos 4 - Kaldena's Resolve
    pub temenos_2020: bool,
    #[default = false]
    /// Temenos 4 - The Path to the Rite
    pub temenos_2030: bool,
    #[default = false]
    /// Temenos 4 - The Nameless Village
    pub temenos_2040: bool,
    #[default = false]
    /// Temenos 4 - Passing the Trial
    pub temenos_2050: bool,
    #[default = false]
    /// Temenos 4 - Guardian Beastling
    pub temenos_2060: bool,
    #[default = false]
    /// Temenos 4 - Kaldena's Rite
    pub temenos_2070: bool,
    #[default = false]
    /// Temenos 4 - The Fallen Godsblades
    pub temenos_2080: bool,
    #[default = false]
    /// Temenos 4 - The Inquisition
    pub temenos_2090: bool,
    #[default = false]
    /// Temenos 4 - Swallowed by Darkness
    pub temenos_2100: bool,
    #[default = false]
    /// Temenos 4 - Case Closed
    pub temenos_2110: bool,
    #[default = false]
    /// Temenos 4 - A Journey Ends, a Stroll Begins
    pub temenos_2500: bool,

    #[default = false]
    /// Partitio Main Story Complete (End Single Story)
    pub partitio_story_complete: bool,
    #[default = false]
    /// Partitio joins the party
    pub partitio_joins: bool,
    #[default = false]
    /// Partitio 1 - There Ain't Scratch
    pub partitio_10: bool,
    #[default = false]
    /// Partitio 1 - A Silver Town
    pub partitio_40: bool,
    #[default = false]
    /// Partitio 1 - Mischief at the Mine
    pub partitio_60: bool,
    #[default = false]
    /// Partitio 1 - The Miners and the Gang
    pub partitio_90: bool,
    #[default = false]
    /// Partitio 1 - The Gang Defeated
    pub partitio_100: bool,
    #[default = false]
    /// Partitio 1 - The Partners Part Ways
    pub partitio_110: bool,
    #[default = false]
    /// Partitio 1 - Your Gaze Is on the Horizon
    pub partitio_120: bool,
    #[default = false]
    /// Partitio 1 - Bedridden Pops
    pub partitio_130: bool,
    #[default = false]
    /// Partitio 1 - Poverty
    pub partitio_150: bool,
    #[default = false]
    /// Partitio 1 - A Word of Advice
    pub partitio_170: bool,
    #[default = false]
    /// Partitio 1 - Take Back the Town
    pub partitio_180: bool,
    #[default = false]
    /// Partitio 1 - Contract Rewritten
    pub partitio_190: bool,
    #[default = false]
    /// Partitio 1 - The Landowner's True Identity
    pub partitio_200: bool,
    #[default = false]
    /// Partitio 1 - The Devil Called Poverty
    pub partitio_500: bool,
    #[default = false]
    /// Partitio 2 - The Eastern Continent
    pub partitio_510: bool,
    #[default = false]
    /// Partitio 2 - Heavy Taxes
    pub partitio_520: bool,
    #[default = false]
    /// Partitio 2 - Ori the Scrivener
    pub partitio_530: bool,
    #[default = false]
    /// Partitio 2 - Lost in Thought
    pub partitio_540: bool,
    #[default = false]
    /// Partitio 2 - Those Worthy of Wealth
    pub partitio_550: bool,
    #[default = false]
    /// Partitio 2 - President Not on the Premises
    pub partitio_560: bool,
    #[default = false]
    /// Partitio 2 - Floyd the Engineer
    pub partitio_580: bool,
    #[default = false]
    /// Partitio 2 - Boiler Material
    pub partitio_600: bool,
    #[default = false]
    /// Partitio 2 - Clockite
    pub partitio_610: bool,
    #[default = false]
    /// Partitio 2 - A Clockmaker's Art
    pub partitio_620: bool,
    #[default = false]
    /// Partitio 2 - New Steam Engine, Complete!
    pub partitio_630: bool,
    #[default = false]
    /// Partitio 2 - A Celebration of Success
    pub partitio_640: bool,
    #[default = false]
    /// Partitio 2 - New Steam Engine, Gone!
    pub partitio_650: bool,
    #[default = false]
    /// Partitio 2 - Garnet, Roque's Beloved Hound
    pub partitio_670: bool,
    #[default = false]
    /// Partitio 2 - An 80-Billion-Leaf Deal
    pub partitio_680: bool,
    #[default = false]
    /// Partitio 2 - The Richest Noble
    pub partitio_690: bool,
    #[default = false]
    /// Partitio 2 - A Promise to Shoeshine Will
    pub partitio_1000: bool,
    #[default = false]
    /// Partitio 3 - Home of the Richest Noble
    pub partitio_1010: bool,
    #[default = false]
    /// Partitio 3 - A Bad Deal Gone Good
    pub partitio_1020: bool,
    #[default = false]
    /// Partitio 3 - Winning an Audience with Alrond
    pub partitio_1030: bool,
    #[default = false]
    /// Partitio 3 - A Distinguished Achievement
    pub partitio_1040: bool,
    #[default = false]
    /// Partitio 3 - Wealthy Noble Alrond
    pub partitio_1050: bool,
    #[default = false]
    /// Partitio 3 - Inspecting the Town
    pub partitio_1060: bool,
    #[default = false]
    /// Partitio 3 - The Ideal Building
    pub partitio_1070: bool,
    #[default = false]
    /// Partitio 3 - Preparing to Open Shop
    pub partitio_1080: bool,
    #[default = false]
    /// Partitio 3 - Procuring the Goods
    pub partitio_1090: bool,
    #[default = false]
    /// Partitio 3 - Alrond's Department Store
    pub partitio_1130: bool,
    #[default = false]
    /// Partitio 3 - White Fog
    pub partitio_1140: bool,
    #[default = false]
    /// Partitio 3 - Manse in the Mist
    pub partitio_1150: bool,
    #[default = false]
    /// Partitio 3 - Hidden in the Fog
    pub partitio_1160: bool,
    #[default = false]
    /// Partitio 3 - Thurston's Loss
    pub partitio_1170: bool,
    #[default = false]
    /// Partitio 3 - 80 Billion to Roque Island
    pub partitio_1500: bool,
    #[default = false]
    /// Partitio 4 - Once-in-a-Lifetime Deal
    pub partitio_1510: bool,
    #[default = false]
    /// Partitio 4 - Roque's Steam Engine
    pub partitio_1520: bool,
    #[default = false]
    /// Partitio 4 - Ori's Big Scoop
    pub partitio_1530: bool,
    #[default = false]
    /// Partitio 4 - Never Underestimate a Scrivener
    pub partitio_1540: bool,
    #[default = false]
    /// Partitio 4 - Don't Look Back!
    pub partitio_1550: bool,
    #[default = false]
    /// Partitio 4 - The Conference Begins
    pub partitio_1560: bool,
    #[default = false]
    /// Partitio 4 - Hold Your Horses!
    pub partitio_1580: bool,
    #[default = false]
    /// Partitio 4 - The 80-Billion-Leaf Man
    pub partitio_1590: bool,
    #[default = false]
    /// Partitio 4 - A Done Deal
    pub partitio_1600: bool,
    #[default = false]
    /// Partitio 4 - Steam Tank Obsidian
    pub partitio_1610: bool,
    #[default = false]
    /// Partitio 4 - His True Desire
    pub partitio_1620: bool,
    #[default = false]
    /// Partitio 4 - Hiring Roque
    pub partitio_1630: bool,
    #[default = false]
    /// Partitio 4 - Partitio & Roque
    pub partitio_1640: bool,
    #[default = false]
    /// Partitio 4 - The Industrial Revolution
    pub partitio_1650: bool,
    #[default = false]
    /// Partitio 4 - Traveling the World
    pub partitio_2000: bool,

    #[default = false]
    /// Osvald Main Story Complete (End Single Story)
    pub osvald_story_complete: bool,
    #[default = false]
    /// Osvald joins the party
    pub osvald_joins: bool,
    #[default = false]
    /// Osvald 1 - The Trial
    pub osvald_10: bool,
    #[default = false]
    /// Osvald 1 - Frigit Isle
    pub osvald_20: bool,
    #[default = false]
    /// Osvald 1 - Osvald's Notebook
    pub osvald_30: bool,
    #[default = false]
    /// Osvald 1 - Warden Davids
    pub osvald_40: bool,
    #[default = false]
    /// Osvald 1 - The Escape Plan
    pub osvald_50: bool,
    #[default = false]
    /// Osvald 1 - Forced Labor
    pub osvald_60: bool,
    #[default = false]
    /// Osvald 1 - Three Pieces
    pub osvald_70: bool,
    #[default = false]
    /// Osvald 1 - Another Dream
    pub osvald_80: bool,
    #[default = false]
    /// Osvald 1 - Researching the One True Magic
    pub osvald_90: bool,
    #[default = false]
    /// Osvald 1 - Awakening
    pub osvald_100: bool,
    #[default = false]
    /// Osvald 1 - A Third Dream
    pub osvald_120: bool,
    #[default = false]
    /// Osvald 1 - Harvey's Scheme
    pub osvald_130: bool,
    #[default = false]
    /// Osvald 1 - A Rude Awakening
    pub osvald_140: bool,
    #[default = false]
    /// Osvald 1 - Report on the Wares
    pub osvald_160: bool,
    #[default = false]
    /// Osvald 1 - The Time Has Come
    pub osvald_180: bool,
    #[default = false]
    /// Osvald 1 - Inspectors at Port
    pub osvald_190: bool,
    #[default = false]
    /// Osvald 1 - A Plan in Motion
    pub osvald_200: bool,
    #[default = false]
    /// Osvald 1 - Muzzle Removed
    pub osvald_210: bool,
    #[default = false]
    /// Osvald 1 - Davids Appears
    pub osvald_220: bool,
    #[default = false]
    /// Osvald 1 - The Answer to Escaping
    pub osvald_500: bool,
    #[default = false]
    /// Osvald 2 - No Answers Today
    pub osvald_510: bool,
    #[default = false]
    /// Osvald 2 - A Raging Fire
    pub osvald_520: bool,
    #[default = false]
    /// Osvald 2 - Escape from the Underground
    pub osvald_530: bool,
    #[default = false]
    /// Osvald 2 - Surveying the Situation
    pub osvald_540: bool,
    #[default = false]
    /// Osvald 2 - On Alert
    pub osvald_550: bool,
    #[default = false]
    /// Osvald 2 - The Answer is Straw
    pub osvald_560: bool,
    #[default = false]
    /// Osvald 2 - A Boat of Straw
    pub osvald_570: bool,
    #[default = false]
    /// Osvald 2 - Parting with Emerald
    pub osvald_580: bool,
    #[default = false]
    /// Osvald 2 - The Inspectors Leave Port
    pub osvald_590: bool,
    #[default = false]
    /// Osvald 2 - An Unsolved Mystery
    pub osvald_600: bool,
    #[default = false]
    /// Osvald 2 - Rita
    pub osvald_610: bool,
    #[default = false]
    /// Osvald 2 - Washing Ashore in Cape Cold
    pub osvald_620: bool,
    #[default = false]
    /// Osvald 2 - The Vengeful Scholar Osvald
    pub osvald_1000: bool,
    #[default = false]
    /// Osvald 3 - All I Had Was Here
    pub osvald_1010: bool,
    #[default = false]
    /// Osvald 3 - I'm Home
    pub osvald_1020: bool,
    #[default = false]
    /// Osvald 3 - Lady Clarissa
    pub osvald_1030: bool,
    #[default = false]
    /// Osvald 3 - Investigating the Incident
    pub osvald_1040: bool,
    #[default = false]
    /// Osvald 3 - Revenge Is Nothing
    pub osvald_1050: bool,
    #[default = false]
    /// Osvald 3 - Stenvar, Captain of the Guard
    pub osvald_1060: bool,
    #[default = false]
    /// Osvald 3 - Harvey's Trail
    pub osvald_1070: bool,
    #[default = false]
    /// Osvald 3 - Harvey's Trap
    pub osvald_1500: bool,
    #[default = false]
    /// Osvald 4 - Meeting Harvey
    pub osvald_1510: bool,
    #[default = false]
    /// Osvald 4 - Montwise, Town of Tomes
    pub osvald_1520: bool,
    #[default = false]
    /// Osvald 4 - Professor Harvey's Lecture
    pub osvald_1530: bool,
    #[default = false]
    /// Osvald 4 - Harvey's Whereabouts
    pub osvald_1540: bool,
    #[default = false]
    /// Osvald 4 - The Library's Hidden Passage
    pub osvald_1560: bool,
    #[default = false]
    /// Osvald 4 - Harvey's Laboratory
    pub osvald_1570: bool,
    #[default = false]
    /// Osvald 4 - The Golem
    pub osvald_1590: bool,
    #[default = false]
    /// Osvald 4 - Elena Lives
    pub osvald_1600: bool,
    #[default = false]
    /// Osvald 4 - Father and Daughter
    pub osvald_1620: bool,
    #[default = false]
    /// Osvald 4 - Osvald's Change
    pub osvald_2000: bool,
    #[default = false]
    /// Osvald 5 - Like Father, Like Daughter
    pub osvald_2010: bool,
    #[default = false]
    /// Osvald 5 - Harvey and Elena
    pub osvald_2020: bool,
    #[default = false]
    /// Osvald 5 - The Seventh Source
    pub osvald_2030: bool,
    #[default = false]
    /// Osvald 5 - Black Crystals
    pub osvald_2040: bool,
    #[default = false]
    /// Osvald 5 - Removing the Crystals
    pub osvald_2050: bool,
    #[default = false]
    /// Osvald 5 - To the Duskruin Shrine
    pub osvald_2070: bool,
    #[default = false]
    /// Osvald 5 - Experiment with the Shadow
    pub osvald_2080: bool,
    #[default = false]
    /// Osvald 5 - The One True Magic
    pub osvald_2090: bool,
    #[default = false]
    /// Osvald 5 - Elena's Research
    pub osvald_2100: bool,
    #[default = false]
    /// Osvald 5 - Osvald's Answer
    pub osvald_2110: bool,
    #[default = false]
    /// Osvald 5 - The One True Magic, Defeated
    pub osvald_2120: bool,
    #[default = false]
    /// Osvald 5 - A Daughter's Hope
    pub osvald_2130: bool,
    #[default = false]
    /// Osvald 5 - An Answer, a Journey
    pub osvald_2500: bool,

    #[default = false]
    /// Ochette Main Story Complete (End Single Story)
    pub ochette_story_complete: bool,
    #[default = false]
    /// Ochette joins the party
    pub ochette_joins: bool,
    #[default = false]
    /// Ochette 1 - A Loyal Companion
    pub ochette_10: bool,
    #[default = false]
    /// Ochette 1 - The One Not Chosen
    pub ochette_30: bool,
    #[default = false]
    /// Ochette 1 - Hunting Together
    pub ochette_40: bool,
    #[default = false]
    /// Ochette 1 - The Way of the Island
    pub ochette_50: bool,
    #[default = false]
    /// Ochette 1 - Greed Betrays You
    pub ochette_60: bool,
    #[default = false]
    /// Ochette 1 - Preparing Food
    pub ochette_70: bool,
    #[default = false]
    /// Ochette 1 - Eating Up
    pub ochette_90: bool,
    #[default = false]
    /// Ochette 1 - Cohazeh's Request
    pub ochette_100: bool,
    #[default = false]
    /// Ochette 1 - Finding the Girl
    pub ochette_130: bool,
    #[default = false]
    /// Ochette 1 - Tasty Jerky
    pub ochette_140: bool,
    #[default = false]
    /// Ochette 1 - The Islebirds' Warning
    pub ochette_150: bool,
    #[default = false]
    /// Ochette 1 - A Village in Danger
    pub ochette_160: bool,
    #[default = false]
    /// Ochette 1 - Inescapable
    pub ochette_170: bool,
    #[default = false]
    /// Ochette 1 - An Ill Omen
    pub ochette_180: bool,
    #[default = false]
    /// Ochette 1 - Three Creatures of Legend
    pub ochette_190: bool,
    #[default = false]
    /// Ochette 1 - Ochette's Mission
    pub ochette_200: bool,
    #[default = false]
    /// Ochette 1 - In Search of Legends
    pub ochette_500: bool,
    #[default = false]
    /// Ochette 2A - The Legend of Cateracta
    pub ochette_510: bool,
    #[default = false]
    /// Ochette 2A - A Strange Sound
    pub ochette_520: bool,
    #[default = false]
    /// Ochette 2A - Alpione, Guardian of the Waves
    pub ochette_530: bool,
    #[default = false]
    /// Ochette 2A - Toward the Sound
    pub ochette_540: bool,
    #[default = false]
    /// Ochette 2A - Bones
    pub ochette_550: bool,
    #[default = false]
    /// Ochette 2A - Cateracta's Last Moments
    pub ochette_560: bool,
    #[default = false]
    /// Ochette 2A - Hatching the Egg
    pub ochette_570: bool,
    #[default = false]
    /// Ochette 2A - Acta
    pub ochette_1000: bool,
    #[default = false]
    /// Ochette 2B - The Legend of Tera
    pub ochette_1010: bool,
    #[default = false]
    /// Ochette 2B - Animal Instincts
    pub ochette_1020: bool,
    #[default = false]
    /// Ochette 2B - Earthquake
    pub ochette_1030: bool,
    #[default = false]
    /// Ochette 2B - Hungry Pom
    pub ochette_1040: bool,
    #[default = false]
    /// Ochette 2B - Buttermeep Jerky
    pub ochette_1050: bool,
    #[default = false]
    /// Ochette 2B - Arrow of Awakening
    pub ochette_1060: bool,
    #[default = false]
    /// Ochette 2B - A Calmed Tera
    pub ochette_1070: bool,
    #[default = false]
    /// Ochette 2B - Pom's Goal
    pub ochette_1500: bool,
    #[default = false]
    /// Ochette 2C - The Legend of Glacis
    pub ochette_1510: bool,
    #[default = false]
    /// Ochette 2C - Glacis's Voice
    pub ochette_1520: bool,
    #[default = false]
    /// Ochette 2C - The Barrier Knight
    pub ochette_1530: bool,
    #[default = false]
    /// Ochette 2C - Sacred Peak Altahe
    pub ochette_1550: bool,
    #[default = false]
    /// Ochette 2C - A Hateful Hunter
    pub ochette_1560: bool,
    #[default = false]
    /// Ochette 2C - The Raging Glacis
    pub ochette_1570: bool,
    #[default = false]
    /// Ochette 2C - Not the Only Thing that Matters
    pub ochette_1580: bool,
    #[default = false]
    /// Ochette 2C - Descending the Mountain
    pub ochette_1590: bool,
    #[default = false]
    /// Ochette 2C - Heig's Confession
    pub ochette_1600: bool,
    #[default = false]
    /// Ochette 2C - A Blizzard Stopped
    pub ochette_2000: bool,
    #[default = false]
    /// Ochette 3 - Ochette's Return
    pub ochette_2020: bool,
    #[default = false]
    /// Ochette 3 - Preparing for Battle
    pub ochette_2030: bool,
    #[default = false]
    /// Ochette 3 - The Shadow Attacks
    pub ochette_2040: bool,
    #[default = false]
    /// Ochette 3 - A Scarlet Sky
    pub ochette_2050: bool,
    #[default = false]
    /// Ochette 3 - The Night of the Scarlet Moon
    pub ochette_2060: bool,
    #[default = false]
    /// Ochette 3 - Unfamiliar Monsters
    pub ochette_2070: bool,
    #[default = false]
    /// Ochette 3 - Beastlings and Humans Unite
    pub ochette_2080: bool,
    #[default = false]
    /// Ochette 3 - Monsters Intercepted
    pub ochette_2090: bool,
    #[default = false]
    /// Ochette 3 - Tera Attacks
    pub ochette_2100: bool,
    #[default = false]
    /// Ochette 3 - Glacis's Snow Storm
    pub ochette_2110: bool,
    #[default = false]
    /// Ochette 3 - A Familiar Scent
    pub ochette_2120: bool,
    #[default = false]
    /// Ochette 3 - On That Day
    pub ochette_2130: bool,
    #[default = false]
    /// Ochette 3 - Into the Ocean
    pub ochette_2140: bool,
    #[default = false]
    /// Ochette 3 - A Flame in the Water
    pub ochette_2150: bool,
    #[default = false]
    /// Ochette 3 - Companion's Awakening
    pub ochette_2160: bool,
    #[default = false]
    /// Ochette 3 - I'll Help You Find Peace
    pub ochette_2170: bool,
    #[default = false]
    /// Ochette 3 - If You're Reborn...
    pub ochette_2180: bool,
    #[default = false]
    /// Ochette 3 - Dawn Breaks
    pub ochette_2190: bool,
    #[default = false]
    /// Ochette 3 - Ochette, Guardian of Toto'haha
    pub ochette_2500: bool,

    #[default = false]
    /// Castti Main Story Complete (End Single Story)
    pub castti_story_complete: bool,
    #[default = false]
    /// Castti joins the party
    pub castti_joins: bool,
    #[default = false]
    /// Castti 1 - Memories
    pub castti_10: bool,
    #[default = false]
    /// Castti 1 - Lost at Sea
    pub castti_20: bool,
    #[default = false]
    /// Castti 1 - Amnesia
    pub castti_30: bool,
    #[default = false]
    /// Castti 1 - Information Gathering
    pub castti_40: bool,
    #[default = false]
    /// Castti 1 - The Satchel
    pub castti_50: bool,
    #[default = false]
    /// Castti 1 - Canalbrine Harbor
    pub castti_60: bool,
    #[default = false]
    /// Castti 1 - A Fallen Boy
    pub castti_70: bool,
    #[default = false]
    /// Castti 1 - Checkup
    pub castti_80: bool,
    #[default = false]
    /// Castti 1 - Mixing Medicine
    pub castti_90: bool,
    #[default = false]
    /// Castti 1 - Eir's Apothecaries
    pub castti_100: bool,
    #[default = false]
    /// Castti 1 - The Ill Repute of Eir's Apothecaries
    pub castti_110: bool,
    #[default = false]
    /// Castti 1 - Discovering the Source
    pub castti_120: bool,
    #[default = false]
    /// Castti 1 - Unsanitary Beasts
    pub castti_140: bool,
    #[default = false]
    /// Castti 1 - Purifying the Waters (Veron + Doron)
    pub castti_150: bool,
    #[default = false]
    /// Castti 1 - A Warm Welcome
    pub castti_160: bool,
    #[default = false]
    /// Castti 1 - Malaya the Apothecary
    pub castti_170: bool,
    #[default = false]
    /// Castti 1 - Memory Fragment: The Pier
    pub castti_180: bool,
    #[default = false]
    /// Castti 1 - Treatment Log
    pub castti_500: bool,
    #[default = false]
    /// Castti 2A - Arrival in Sai
    pub castti_510: bool,
    #[default = false]
    /// Castti 2A - Master Edmund
    pub castti_520: bool,
    #[default = false]
    /// Castti 2A - Mao the Bonemender
    pub castti_530: bool,
    #[default = false]
    /// Castti 2A - Memory Fragment: Sai's Hospital
    pub castti_540: bool,
    #[default = false]
    /// Castti 2A - The Real Eir's Apothecaries
    pub castti_550: bool,
    #[default = false]
    /// Castti 2A - Friend or Foe?
    pub castti_560: bool,
    #[default = false]
    /// Castti 2A - Helping the Injured
    pub castti_570: bool,
    #[default = false]
    /// Castti 2A - The Monster of the Sands
    pub castti_580: bool,
    #[default = false]
    /// Castti 2A - Sinking Sands
    pub castti_590: bool,
    #[default = false]
    /// Castti 2A - The Lion's Den
    pub castti_600: bool,
    #[default = false]
    /// Castti 2A - Finding the Sand Lion
    pub castti_610: bool,
    #[default = false]
    /// Castti 2A - Dealing with the Sand Lion
    pub castti_620: bool,
    #[default = false]
    /// Castti 2A - Accord
    pub castti_630: bool,
    #[default = false]
    /// Castti 2A - Rumors of Eir's Apothecaries
    pub castti_640: bool,
    #[default = false]
    /// Castti 2A - Memory Fragment: A Rainy Village
    pub castti_650: bool,
    #[default = false]
    /// Castti 2A - Memory Fragment: Rain Man
    pub castti_660: bool,
    #[default = false]
    /// Castti 2A - In Search of Fresh Clues
    pub castti_1000: bool,
    #[default = false]
    /// Castti 2B - Arrival in Winterbloom
    pub castti_1010: bool,
    #[default = false]
    /// Castti 2B - Gratitude for Eir's Apothecaries
    pub castti_1020: bool,
    #[default = false]
    /// Castti 2B - The Lady's Manor
    pub castti_1030: bool,
    #[default = false]
    /// Castti 2B - Lady Rosa
    pub castti_1040: bool,
    #[default = false]
    /// Castti 2B - The Herb Garden
    pub castti_1050: bool,
    #[default = false]
    /// Castti 2B - Melia's Wish
    pub castti_1070: bool,
    #[default = false]
    /// Castti 2B - Greg, Usurper Aspirant
    pub castti_1080: bool,
    #[default = false]
    /// Castti 2B - Even with Memories Lost
    pub castti_1090: bool,
    #[default = false]
    /// Castti 2B - Melia Kidnapped
    pub castti_1100: bool,
    #[default = false]
    /// Castti 2B - Greg's Ambitions
    pub castti_1110: bool,
    #[default = false]
    /// Castti 2B - Rogue Conflict
    pub castti_1120: bool,
    #[default = false]
    /// Castti 2B - Twelfth Birthday
    pub castti_1130: bool,
    #[default = false]
    /// Castti 2B - Memory Fragment: A Promise to Rosa
    pub castti_1140: bool,
    #[default = false]
    /// Castti 2B - Rosa's Last Breath
    pub castti_1150: bool,
    #[default = false]
    /// Castti 2B - Trousseau
    pub castti_1160: bool,
    #[default = false]
    /// Castti 2B - Memory Fragment: Trousseau
    pub castti_1170: bool,
    #[default = false]
    /// Castti 2B - In Search of Fresh Clues
    pub castti_1500: bool,
    #[default = false]
    /// Castti 3 - Ghost Town
    pub castti_1510: bool,
    #[default = false]
    /// Castti 3 - Santanejo
    pub castti_1520: bool,
    #[default = false]
    /// Castti 3 - Memory Fragment: Once Upon a Time
    pub castti_1530: bool,
    #[default = false]
    /// Castti 3 - Memory Fragment: A Sick Girl
    pub castti_1540: bool,
    #[default = false]
    /// Castti 3 - Memory Fragment: Searching for Santanejo
    pub castti_1550: bool,
    #[default = false]
    /// Castti 3 - Memory Fragment: Trousseau's Dreams
    pub castti_1560: bool,
    #[default = false]
    /// Castti 3 - What Happened Here?
    pub castti_1570: bool,
    #[default = false]
    /// Castti 3 - Memory Fragment: Disturbing Signs
    pub castti_1580: bool,
    #[default = false]
    /// Castti 3 - Memory Fragment: Purple Rain
    pub castti_1590: bool,
    #[default = false]
    /// Castti 3 - Memory Fragment: The Summit
    pub castti_1600: bool,
    #[default = false]
    /// Castti 3 - Memories of Rain
    pub castti_1610: bool,
    #[default = false]
    /// Castti 3 - Memory Fragment: The Smoke's Source
    pub castti_1620: bool,
    #[default = false]
    /// Castti 3 - Memory Fragment: Bitter Partings
    pub castti_1630: bool,
    #[default = false]
    /// Castti 3 - Memory Fragment: To the Village
    pub castti_1640: bool,
    #[default = false]
    /// Castti 3 - Memory Fragment: Escape
    pub castti_1650: bool,
    #[default = false]
    /// Castti 3 - Memory Fragment: Hope Entrusted
    pub castti_1660: bool,
    #[default = false]
    /// Castti 3 - The Truth About Malaya
    pub castti_2000: bool,
    #[default = false]
    /// Castti 4 - The Coronation Ceremony
    pub castti_2010: bool,
    #[default = false]
    /// Castti 4 - Reuniting with Edmund
    pub castti_2020: bool,
    #[default = false]
    /// Castti 4 - Griff's Support
    pub castti_2030: bool,
    #[default = false]
    /// Castti 4 - Rain, Rain
    pub castti_2040: bool,
    #[default = false]
    /// Castti 4 - Rain to Purple
    pub castti_2050: bool,
    #[default = false]
    /// Castti 4 - To Extend a Helping Hand
    pub castti_2060: bool,
    #[default = false]
    /// Castti 4 - Concocting a Cure
    pub castti_2070: bool,
    #[default = false]
    /// Castti 4 - Miracle Cure
    pub castti_2080: bool,
    #[default = false]
    /// Castti 4 - Healing Those in Need
    pub castti_2090: bool,
    #[default = false]
    /// Castti 4 - Castti Collapses
    pub castti_2100: bool,
    #[default = false]
    /// Castti 4 - Malaya's Request
    pub castti_2110: bool,
    #[default = false]
    /// Castti 4 - Partings
    pub castti_2120: bool,
    #[default = false]
    /// Castti 4 - The Deeds of Eir's Apothecaries
    pub castti_2500: bool,

    #[default = false]
    /// Hikari Main Story Complete (End Single Story)
    pub hikari_story_complete: bool,
    #[default = false]
    /// Hikari joins the party
    pub hikari_joins: bool,
    #[default = false]
    /// Hikari 1 - Before the Battle, Before Their Graves
    pub hikari_10: bool,
    #[default = false]
    /// Hikari 1 - General Mugen
    pub hikari_30: bool,
    #[default = false]
    /// Hikari 1 - Advance
    pub hikari_50: bool,
    #[default = false]
    /// Hikari 1 - Clash with the Enemy General
    pub hikari_60: bool,
    #[default = false]
    /// Hikari 1 - Ruler of the Desert
    pub hikari_70: bool,
    #[default = false]
    /// Hikari 1 - Friendly Banter
    pub hikari_80: bool,
    #[default = false]
    /// Hikari 1 - Peace Yet Prevails
    pub hikari_90: bool,
    #[default = false]
    /// Hikari 1 - Tussle at the Tavern
    pub hikari_100: bool,
    #[default = false]
    /// Hikari 1 - The Tavern Abuzz
    pub hikari_120: bool,
    #[default = false]
    /// Hikari 1 - An Unexpected Visitor
    pub hikari_130: bool,
    #[default = false]
    /// Hikari 1 - Jigo, King and Father
    pub hikari_140: bool,
    #[default = false]
    /// Hikari 1 - Merchant Found
    pub hikari_150: bool,
    #[default = false]
    /// Hikari 1 - Disquieting News
    pub hikari_170: bool,
    #[default = false]
    /// Hikari 1 - In the Throne Room
    pub hikari_190: bool,
    #[default = false]
    /// Hikari 1 - At the Crest of Heroes
    pub hikari_200: bool,
    #[default = false]
    /// Hikari 1 - A Sinister Voice
    pub hikari_210: bool,
    #[default = false]
    /// Hikari 1 - The Death of a Friend
    pub hikari_220: bool,
    #[default = false]
    /// Hikari 1 - Mugen's Treachery
    pub hikari_230: bool,
    #[default = false]
    /// Hikari 1 - Hikari's Resolve
    pub hikari_240: bool,
    #[default = false]
    /// Hikari 1 - Leaving Home Behind
    pub hikari_250: bool,
    #[default = false]
    /// Hikari 1 - In Search of Allies
    pub hikari_500: bool,
    #[default = false]
    /// Hikari 2 - The Eagle's Perch
    pub hikari_510: bool,
    #[default = false]
    /// Hikari 2 - An Unexpected Reunion
    pub hikari_520: bool,
    #[default = false]
    /// Hikari 2 - A New Challenger
    pub hikari_530: bool,
    #[default = false]
    /// Hikari 2 - Kazan's Vices
    pub hikari_540: bool,
    #[default = false]
    /// Hikari 2 - Bandelam the Reaper
    pub hikari_550: bool,
    #[default = false]
    /// Hikari 2 - Duels to the Death
    pub hikari_560: bool,
    #[default = false]
    /// Hikari 2 - The Challenge Begins
    pub hikari_580: bool,
    #[default = false]
    /// Hikari 2 - Qualifying for the Duel
    pub hikari_590: bool,
    #[default = false]
    /// Hikari 2 - Master Borneau
    pub hikari_600: bool,
    #[default = false]
    /// Hikari 2 - A Steep Wager
    pub hikari_630: bool,
    #[default = false]
    /// Hikari 2 - Zeto the Butcher
    pub hikari_640: bool,
    #[default = false]
    /// Hikari 2 - The Way of the Arena
    pub hikari_650: bool,
    #[default = false]
    /// Hikari 2 - Kazan's Lie
    pub hikari_660: bool,
    #[default = false]
    /// Hikari 2 - 300 Million Leaves
    pub hikari_670: bool,
    #[default = false]
    /// Hikari 2 - Duel with Bandelam
    pub hikari_690: bool,
    #[default = false]
    /// Hikari 2 - A Reason to Fight
    pub hikari_700: bool,
    #[default = false]
    /// Hikari 2 - The Gladiators Revolt
    pub hikari_710: bool,
    #[default = false]
    /// Hikari 2 - Gladiators Freed
    pub hikari_730: bool,
    #[default = false]
    /// Hikari 2 - New Friends
    pub hikari_1000: bool,
    #[default = false]
    /// Hikari 3 - The Sky That Day
    pub hikari_1010: bool,
    #[default = false]
    /// Hikari 3 - Meeting Ritsu
    pub hikari_1020: bool,
    #[default = false]
    /// Hikari 3 - Searching for Azuma
    pub hikari_1030: bool,
    #[default = false]
    /// Hikari 3 - The Exchange
    pub hikari_1040: bool,
    #[default = false]
    /// Hikari 3 - An Important Mission
    pub hikari_1050: bool,
    #[default = false]
    /// Hikari 3 - Weapon Transport Interrupted
    pub hikari_1060: bool,
    #[default = false]
    /// Hikari 3 - General Rou
    pub hikari_1070: bool,
    #[default = false]
    /// Hikari 3 - A Looming Shadow
    pub hikari_1080: bool,
    #[default = false]
    /// Hikari 3 - Ritsu's Ambition
    pub hikari_1090: bool,
    #[default = false]
    /// Hikari 3 - Weapons Seized
    pub hikari_1100: bool,
    #[default = false]
    /// Hikari 3 - Persuading Rai Mei
    pub hikari_1110: bool,
    #[default = false]
    /// Hikari 3 - Ritsu's Lie
    pub hikari_1500: bool,
    #[default = false]
    /// Hikari 4 - Mugen's Plan
    pub hikari_1510: bool,
    #[default = false]
    /// Hikari 4 - To Castle Mei
    pub hikari_1520: bool,
    #[default = false]
    /// Hikari 4 - Kunzo the Obstructor
    pub hikari_1530: bool,
    #[default = false]
    /// Hikari 4 - Mugen's Vassals
    pub hikari_1540: bool,
    #[default = false]
    /// Hikari 4 - Mother's Voice
    pub hikari_1550: bool,
    #[default = false]
    /// Hikari 4 - Meeting Clan Mei
    pub hikari_1560: bool,
    #[default = false]
    /// Hikari 4 - Hikari's Talents
    pub hikari_1570: bool,
    #[default = false]
    /// Hikari 4 - Clan Mei Moves Out
    pub hikari_1580: bool,
    #[default = false]
    /// Hikari 4 - A Kind Mother's Demise
    pub hikari_1590: bool,
    #[default = false]
    /// Hikari 4 - Hikari Loses Control
    pub hikari_1600: bool,
    #[default = false]
    /// Hikari 4 - A Death Sentence
    pub hikari_1610: bool,
    #[default = false]
    /// Hikari 4 - The Accursed Clan
    pub hikari_1620: bool,
    #[default = false]
    /// Hikari 4 - A Startling Truth
    pub hikari_1630: bool,
    #[default = false]
    /// Hikari 4 - Clan Leader Jin Mei
    pub hikari_1640: bool,
    #[default = false]
    /// Hikari 4 - Rai Mei's Resolve
    pub hikari_1650: bool,
    #[default = false]
    /// Hikari 4 - For Clan Mei's Survival
    pub hikari_1660: bool,
    #[default = false]
    /// Hikari 4 - Kunzo's Loyalty
    pub hikari_1670: bool,
    #[default = false]
    /// Hikari 4 - The Paths We Make
    pub hikari_1680: bool,
    #[default = false]
    /// Hikari 4 - I'll Await You in Ku
    pub hikari_2000: bool,
    #[default = false]
    /// Hikari 5 - The Prince Returns
    pub hikari_2010: bool,
    #[default = false]
    /// Hikari 5 - Ritsu and Mikka
    pub hikari_2020: bool,
    #[default = false]
    /// Hikari 5 - General Ritsu
    pub hikari_2030: bool,
    #[default = false]
    /// Hikari 5 - The Eagle's Invitation
    pub hikari_2040: bool,
    #[default = false]
    /// Hikari 5 - A Gathering of Friends
    pub hikari_2050: bool,
    #[default = false]
    /// Hikari 5 - The Sandstorm
    pub hikari_2060: bool,
    #[default = false]
    /// Hikari 5 - The Bell of Dawn
    pub hikari_2070: bool,
    #[default = false]
    /// Hikari 5 - Mugen's Order
    pub hikari_2080: bool,
    #[default = false]
    /// Hikari 5 - Ageha's Trap
    pub hikari_2090: bool,
    #[default = false]
    /// Hikari 5 - Kazan's Plan
    pub hikari_2100: bool,
    #[default = false]
    /// Hikari 5 - Rai Mei, Spear of Levin
    pub hikari_2110: bool,
    #[default = false]
    /// Hikari 5 - Diverging Paths
    pub hikari_2120: bool,
    #[default = false]
    /// Hikari 5 - The Plea of Friends
    pub hikari_2130: bool,
    #[default = false]
    /// Hikari 5 - Brothers Reunited
    pub hikari_2150: bool,
    #[default = false]
    /// Hikari 5 - Hikari's Blood
    pub hikari_2160: bool,
    #[default = false]
    /// Hikari 5 - The Darkblood Blade
    pub hikari_2170: bool,
    #[default = false]
    /// Hikari 5 - No Longer Consumed by Darkness
    pub hikari_2180: bool,
    #[default = false]
    /// Hikari 5 - A Joyous Proclamation
    pub hikari_2190: bool,
    #[default = false]
    /// Hikari 5 - Ascending the Throne
    pub hikari_2200: bool,
    #[default = false]
    /// Hikari 5 - The Blessing of Friendship
    pub hikari_2210: bool,
    #[default = false]
    /// Hikari 5 - King Hikari of Ku
    pub hikari_2220: bool,
    #[default = false]
    /// Hikari 5 - Clear Skies
    pub hikari_2500: bool,

    #[default = false]
    /// Agnea Main Story Complete (End Single Story)
    pub agnea_story_complete: bool,
    #[default = false]
    /// Agnea joins the party
    pub agnea_joins: bool,
    #[default = false]
    /// Agnea 1 - Dreams of Stardom
    pub agnea_10: bool,
    #[default = false]
    /// Agnea 1 - The Village Dancer
    pub agnea_20: bool,
    #[default = false]
    /// Agnea 1 - Seeing Off the Patrons
    pub agnea_30: bool,
    #[default = false]
    /// Agnea 1 - The Tavern After Closing
    pub agnea_40: bool,
    #[default = false]
    /// Agnea 1 - Dancing on Air
    pub agnea_50: bool,
    #[default = false]
    /// Agnea 1 - Telling Papa the News
    pub agnea_60: bool,
    #[default = false]
    /// Agnea 1 - Cuani the Star
    pub agnea_70: bool,
    #[default = false]
    /// Agnea 1 - Memories of Mama
    pub agnea_80: bool,
    #[default = false]
    /// Agnea 1 - Nothing Special
    pub agnea_90: bool,
    #[default = false]
    /// Agnea 1 - Preparing for the Festival
    pub agnea_100: bool,
    #[default = false]
    /// Agnea 1 - Pala's Gone Missing
    pub agnea_110: bool,
    #[default = false]
    /// Agnea 1 - The Raging Duorduor
    pub agnea_120: bool,
    #[default = false]
    /// Agnea 1 - A Dress Ruined
    pub agnea_130: bool,
    #[default = false]
    /// Agnea 1 - I Can Still Dance
    pub agnea_140: bool,
    #[default = false]
    /// Agnea 1 - Mama's Dress
    pub agnea_150: bool,
    #[default = false]
    /// Agnea 1 - Mama's Song
    pub agnea_160: bool,
    #[default = false]
    /// Agnea 1 - The Morning of Departure
    pub agnea_170: bool,
    #[default = false]
    /// Agnea 1 - We Will Meet Again Someday
    pub agnea_500: bool,
    #[default = false]
    /// Agnea 2 - The Metropolis
    pub agnea_510: bool,
    #[default = false]
    /// Agnea 2 - The Theater
    pub agnea_520: bool,
    #[default = false]
    /// Agnea 2 - Into the Theater
    pub agnea_550: bool,
    #[default = false]
    /// Agnea 2 - The Show Begins
    pub agnea_560: bool,
    #[default = false]
    /// Agnea 2 - Greatest Dancer in the Land
    pub agnea_570: bool,
    #[default = false]
    /// Agnea 2 - Gil the Pianist
    pub agnea_580: bool,
    #[default = false]
    /// Agnea 2 - Montraine's Tavern
    pub agnea_590: bool,
    #[default = false]
    /// Agnea 2 - A Show at the Tavern
    pub agnea_600: bool,
    #[default = false]
    /// Agnea 2 - Agnea the House Dancer
    pub agnea_610: bool,
    #[default = false]
    /// Agnea 2 - An Arrogant Man
    pub agnea_620: bool,
    #[default = false]
    /// Agnea 2 - Shattered Hopes
    pub agnea_630: bool,
    #[default = false]
    /// Agnea 2 - Finding the Manager
    pub agnea_640: bool,
    #[default = false]
    /// Agnea 2 - Manager Found
    pub agnea_650: bool,
    #[default = false]
    /// Agnea 2 - For Hope
    pub agnea_660: bool,
    #[default = false]
    /// Agnea 2 - Shining Superstar
    pub agnea_670: bool,
    #[default = false]
    /// Agnea 2 - Hope Protected
    pub agnea_680: bool,
    #[default = false]
    /// Agnea 2 - A Gift From Gil
    pub agnea_690: bool,
    #[default = false]
    /// Agnea 2 - Song of Hope
    pub agnea_700: bool,
    #[default = false]
    /// Agnea 2 - Dolcinaea's Plan
    pub agnea_1000: bool,
    #[default = false]
    /// Agnea 3 - The Traveling Troupe
    pub agnea_1010: bool,
    #[default = false]
    /// Agnea 3 - Giselle's Troupe
    pub agnea_1030: bool,
    #[default = false]
    /// Agnea 3 - The Missing Troupe Leader
    pub agnea_1040: bool,
    #[default = false]
    /// Agnea 3 - The Pressure of the Stage
    pub agnea_1050: bool,
    #[default = false]
    /// Agnea 3 - Even If You Stumble
    pub agnea_1060: bool,
    #[default = false]
    /// Agnea 3 - The Show Goes On
    pub agnea_1070: bool,
    #[default = false]
    /// Agnea 3 - A Celebratory Drink
    pub agnea_1080: bool,
    #[default = false]
    /// Agnea 3 - Promises of Reunion
    pub agnea_1500: bool,
    #[default = false]
    /// Agnea 4 - Where Mama Was
    pub agnea_1510: bool,
    #[default = false]
    /// Agnea 4 - A Mischievous Child
    pub agnea_1520: bool,
    #[default = false]
    /// Agnea 4 - Hope at Risk
    pub agnea_1530: bool,
    #[default = false]
    /// Agnea 4 - Laila Leaves Home
    pub agnea_1540: bool,
    #[default = false]
    /// Agnea 4 - Having Fun
    pub agnea_1550: bool,
    #[default = false]
    /// Agnea 4 - In Step
    pub agnea_1580: bool,
    #[default = false]
    /// Agnea 4 - The Joy of Dancing
    pub agnea_1590: bool,
    #[default = false]
    /// Agnea 4 - Cuani and Dolcinaea
    pub agnea_1600: bool,
    #[default = false]
    /// Agnea 4 - A Town in Trouble
    pub agnea_1610: bool,
    #[default = false]
    /// Agnea 4 - No Mercy
    pub agnea_1620: bool,
    #[default = false]
    /// Agnea 4 - Dolcinaealand
    pub agnea_1630: bool,
    #[default = false]
    /// Agnea 4 - A True Star
    pub agnea_1640: bool,
    #[default = false]
    /// Agnea 4 - Invitation to the Grand Gala
    pub agnea_1650: bool,
    #[default = false]
    /// Agnea 4 - Laila's Dance
    pub agnea_1660: bool,
    #[default = false]
    /// Agnea 4 - Parting from Laila
    pub agnea_2000: bool,
    #[default = false]
    /// Agnea 5 - Center of Attention
    pub agnea_2010: bool,
    #[default = false]
    /// Agnea 5 - To the Grand Gala's Stage
    pub agnea_2020: bool,
    #[default = false]
    /// Agnea 5 - The Mastermind
    pub agnea_2030: bool,
    #[default = false]
    /// Agnea 5 - Reunited with Giselle's Troupe
    pub agnea_2040: bool,
    #[default = false]
    /// Agnea 5 - Rush to the Stage
    pub agnea_2050: bool,
    #[default = false]
    /// Agnea 5 - The Grandeur of the Stage
    pub agnea_2060: bool,
    #[default = false]
    /// Agnea 5 - Gil and Laila's Show
    pub agnea_2080: bool,
    #[default = false]
    /// Agnea 5 - A Bothered Bodyguard, a Star at Ease
    pub agnea_2090: bool,
    #[default = false]
    /// Agnea 5 - The Manager's Resentment
    pub agnea_2100: bool,
    #[default = false]
    /// Agnea 5 - The Festival of Grace
    pub agnea_2110: bool,
    #[default = false]
    /// Agnea 5 - Showtime
    pub agnea_2120: bool,
    #[default = false]
    /// Agnea 5 - Hope Rekindled
    pub agnea_2130: bool,
    #[default = false]
    /// Agnea 5 - At the Dance Battle's End
    pub agnea_2140: bool,
    #[default = false]
    /// Agnea 5 - A Star without Shine
    pub agnea_2150: bool,
    #[default = false]
    /// Agnea 5 - Dolcinaea's Defeat
    pub agnea_2160: bool,
    #[default = false]
    /// Agnea 5 - Encore
    pub agnea_2170: bool,
    #[default = false]
    /// Agnea 5 - Agnea the Star
    pub agnea_2500: bool,

    // Levels
    #[default = false]
    /// Enter Abandoned Church
    pub dng_mnt_3_3_enter: bool,

    #[default = false]
    /// Exit Abandoned Church
    pub dng_mnt_3_3_exit: bool,

    #[default = false]
    /// Enter Abandoned Road
    pub fld_wld_2_3_enter: bool,

    #[default = false]
    /// Exit Abandoned Road
    pub fld_wld_2_3_exit: bool,

    #[default = false]
    /// Enter Abandoned Silver Mine
    pub fld_wld_1_1_b_enter: bool,

    #[default = false]
    /// Exit Abandoned Silver Mine
    pub fld_wld_1_1_b_exit: bool,

    #[default = false]
    /// Enter Abandoned Traverse
    pub fld_mnt_3_3_enter: bool,

    #[default = false]
    /// Exit Abandoned Traverse
    pub fld_mnt_3_3_exit: bool,

    #[default = false]
    /// Enter Abandoned Village
    pub twn_cty_1_2_a_enter: bool,

    #[default = false]
    /// Exit Abandoned Village
    pub twn_cty_1_2_a_exit: bool,

    #[default = false]
    /// Enter Abandoned Waterway
    pub dng_cty_1_3_enter: bool,

    #[default = false]
    /// Exit Abandoned Waterway
    pub dng_cty_1_3_exit: bool,

    #[default = false]
    /// Enter Abyssal Beach
    pub dng_isd_3_2_e_enter: bool,

    #[default = false]
    /// Exit Abyssal Beach
    pub dng_isd_3_2_e_exit: bool,

    #[default = false]
    /// Enter Altar of the Charitable
    pub dng_sea_2_job_enter: bool,

    #[default = false]
    /// Exit Altar of the Charitable
    pub dng_sea_2_job_exit: bool,

    #[default = false]
    /// Enter Altar of the Flamebringer
    pub dng_mnt_2_job_enter: bool,

    #[default = false]
    /// Exit Altar of the Flamebringer
    pub dng_mnt_2_job_exit: bool,

    #[default = false]
    /// Enter Altar of the Huntress
    pub dng_isd_2_job_enter: bool,

    #[default = false]
    /// Exit Altar of the Huntress
    pub dng_isd_2_job_exit: bool,

    #[default = false]
    /// Enter Altar of the Lady of Grace
    pub dng_fst_2_job_enter: bool,

    #[default = false]
    /// Exit Altar of the Lady of Grace
    pub dng_fst_2_job_exit: bool,

    #[default = false]
    /// Enter Altar of the Prince of Thieves
    pub dng_cty_2_job_enter: bool,

    #[default = false]
    /// Exit Altar of the Prince of Thieves
    pub dng_cty_2_job_exit: bool,

    #[default = false]
    /// Enter Altar of the Scholarking
    pub dng_snw_2_job_enter: bool,

    #[default = false]
    /// Exit Altar of the Scholarking
    pub dng_snw_2_job_exit: bool,

    #[default = false]
    /// Enter Altar of the Thunderblade
    pub dng_dst_2_job_enter: bool,

    #[default = false]
    /// Exit Altar of the Thunderblade
    pub dng_dst_2_job_exit: bool,

    #[default = false]
    /// Enter Altar of the Trader
    pub dng_wld_2_job_enter: bool,

    #[default = false]
    /// Exit Altar of the Trader
    pub dng_wld_2_job_exit: bool,

    #[default = false]
    /// Enter Animal Trail
    pub dng_fst_2_2_a_enter: bool,

    #[default = false]
    /// Exit Animal Trail
    pub dng_fst_2_2_a_exit: bool,

    #[default = false]
    /// Enter Battlefield
    pub fld_dst_3_1_a_enter: bool,

    #[default = false]
    /// Exit Battlefield
    pub fld_dst_3_1_a_exit: bool,

    #[default = false]
    /// Enter Beasting Bay: Anchorage
    pub fld_isd_1_3_enter: bool,

    #[default = false]
    /// Exit Beasting Bay: Anchorage
    pub fld_isd_1_3_exit: bool,

    #[default = false]
    /// Enter Beasting Village
    pub twn_isd_1_1_a_enter: bool,

    #[default = false]
    /// Exit Beasting Village
    pub twn_isd_1_1_a_exit: bool,

    #[default = false]
    /// Enter Bed of the Titan
    pub dng_wld_2_1_b_enter: bool,

    #[default = false]
    /// Exit Bed of the Titan
    pub dng_wld_2_1_b_exit: bool,

    #[default = false]
    /// Enter Beneath the Wall
    pub dng_snw_3_4_a_enter: bool,

    #[default = false]
    /// Exit Beneath the Wall
    pub dng_snw_3_4_a_exit: bool,

    #[default = false]
    /// Enter Borderfall
    pub fld_mnt_2_2_enter: bool,

    #[default = false]
    /// Exit Borderfall
    pub fld_mnt_2_2_exit: bool,

    #[default = false]
    /// Enter Canalbrine
    pub twn_sea_1_1_a_enter: bool,

    #[default = false]
    /// Exit Canalbrine
    pub twn_sea_1_1_a_exit: bool,

    #[default = false]
    /// Enter Canalbrine Bridge
    pub fld_sea_1_2_enter: bool,

    #[default = false]
    /// Exit Canalbrine Bridge
    pub fld_sea_1_2_exit: bool,

    #[default = false]
    /// Enter Canalbrine: Path to the Water Source
    pub fld_sea_1_1_enter: bool,

    #[default = false]
    /// Exit Canalbrine: Path to the Water Source
    pub fld_sea_1_1_exit: bool,

    #[default = false]
    /// Enter Canalbrine: Water Source
    pub dng_sea_1_1_enter: bool,

    #[default = false]
    /// Exit Canalbrine: Water Source
    pub dng_sea_1_1_exit: bool,

    #[default = false]
    /// Enter Cape Cold
    pub twn_snw_1_2_a_enter: bool,

    #[default = false]
    /// Exit Cape Cold
    pub twn_snw_1_2_a_exit: bool,

    #[default = false]
    /// Enter Castle Ku
    pub twn_dst_3_1_c_enter: bool,

    #[default = false]
    /// Exit Castle Ku
    pub twn_dst_3_1_c_exit: bool,

    #[default = false]
    /// Enter Castle Ku: Entrance
    pub twn_dst_3_1_b_enter: bool,

    #[default = false]
    /// Exit Castle Ku: Entrance
    pub twn_dst_3_1_b_exit: bool,

    #[default = false]
    /// Enter Castle Mei: East Tower
    pub dng_snw_3_2_a_enter: bool,

    #[default = false]
    /// Exit Castle Mei: East Tower
    pub dng_snw_3_2_a_exit: bool,

    #[default = false]
    /// Enter Castle Mei: Gallows
    pub dng_snw_3_2_b_enter: bool,

    #[default = false]
    /// Exit Castle Mei: Gallows
    pub dng_snw_3_2_b_exit: bool,

    #[default = false]
    /// Enter Castle Vidania
    pub dng_atl_3_1_enter: bool,

    #[default = false]
    /// Exit Castle Vidania
    pub dng_atl_3_1_exit: bool,

    #[default = false]
    /// Enter Cathedral Cellars
    pub dng_mnt_1_1_enter: bool,

    #[default = false]
    /// Exit Cathedral Cellars
    pub dng_mnt_1_1_exit: bool,

    #[default = false]
    /// Enter Cavern of the Moon and Sun
    pub dng_sea_2_3_enter: bool,

    #[default = false]
    /// Exit Cavern of the Moon and Sun
    pub dng_sea_2_3_exit: bool,

    #[default = false]
    /// Enter Cavern of the Sea God
    pub dng_sea_2_1_enter: bool,

    #[default = false]
    /// Exit Cavern of the Sea God
    pub dng_sea_2_1_exit: bool,

    #[default = false]
    /// Enter Cavern of Waves
    pub dng_isd_1_2_enter: bool,

    #[default = false]
    /// Exit Cavern of Waves
    pub dng_isd_1_2_exit: bool,

    #[default = false]
    /// Enter Clockbank
    pub twn_cty_2_1_a_enter: bool,

    #[default = false]
    /// Exit Clockbank
    pub twn_cty_2_1_a_exit: bool,

    #[default = false]
    /// Enter Clockbank: Industrial District
    pub twn_cty_2_1_b_enter: bool,

    #[default = false]
    /// Exit Clockbank: Industrial District
    pub twn_cty_2_1_b_exit: bool,

    #[default = false]
    /// Enter Conning Creek
    pub twn_sea_2_1_a_enter: bool,

    #[default = false]
    /// Exit Conning Creek
    pub twn_sea_2_1_a_exit: bool,

    #[default = false]
    /// Enter Conning Creek: Harbor
    pub twn_sea_2_1_b_enter: bool,

    #[default = false]
    /// Exit Conning Creek: Harbor
    pub twn_sea_2_1_b_exit: bool,

    #[default = false]
    /// Enter Conning Creek: Outskirts
    pub twn_sea_2_1_c_enter: bool,

    #[default = false]
    /// Exit Conning Creek: Outskirts
    pub twn_sea_2_1_c_exit: bool,

    #[default = false]
    /// Enter Crackridge
    pub twn_wld_2_1_a_enter: bool,

    #[default = false]
    /// Exit Crackridge
    pub twn_wld_2_1_a_exit: bool,

    #[default = false]
    /// Enter Crackridge Harbor: Anchorage
    pub fld_wld_1_2_enter: bool,

    #[default = false]
    /// Exit Crackridge Harbor: Anchorage
    pub fld_wld_1_2_exit: bool,

    #[default = false]
    /// Enter Cropdale
    pub twn_fst_1_1_a_enter: bool,

    #[default = false]
    /// Exit Cropdale
    pub twn_fst_1_1_a_exit: bool,

    #[default = false]
    /// Enter Curious Nest
    pub dng_ocn_1_2_enter: bool,

    #[default = false]
    /// Exit Curious Nest
    pub dng_ocn_1_2_exit: bool,

    #[default = false]
    /// Enter Dark Night
    pub dng_fst_2_2_b_enter: bool,

    #[default = false]
    /// Exit Dark Night
    pub dng_fst_2_2_b_exit: bool,

    #[default = false]
    /// Enter Decaying Temple
    pub dng_dst_2_3_enter: bool,

    #[default = false]
    /// Exit Decaying Temple
    pub dng_dst_2_3_exit: bool,

    #[default = false]
    /// Enter Deserted Highroad
    pub fld_cty_3_1_enter: bool,

    #[default = false]
    /// Exit Deserted Highroad
    pub fld_cty_3_1_exit: bool,

    #[default = false]
    /// Enter Diamante's Estate
    pub dng_cty_1_1_enter: bool,

    #[default = false]
    /// Exit Diamante's Estate
    pub dng_cty_1_1_exit: bool,

    #[default = false]
    /// Enter Dragonridge
    pub dng_dst_2_1_enter: bool,

    #[default = false]
    /// Exit Dragonridge
    pub dng_dst_2_1_exit: bool,

    #[default = false]
    /// Enter Duskruin Shrine
    pub dng_wld_3_1_a_enter: bool,

    #[default = false]
    /// Exit Duskruin Shrine
    pub dng_wld_3_1_a_exit: bool,

    #[default = false]
    /// Enter Duskruin Shrine: Depths
    pub dng_wld_3_1_b_enter: bool,

    #[default = false]
    /// Exit Duskruin Shrine: Depths
    pub dng_wld_3_1_b_exit: bool,

    #[default = false]
    /// Enter Eastern Cape Cold Snows
    pub fld_snw_1_3_enter: bool,

    #[default = false]
    /// Exit Eastern Cape Cold Snows
    pub fld_snw_1_3_exit: bool,

    #[default = false]
    /// Enter Eastern Cropdale Trail
    pub fld_fst_1_3_enter: bool,

    #[default = false]
    /// Exit Eastern Cropdale Trail
    pub fld_fst_1_3_exit: bool,

    #[default = false]
    /// Enter Eastern Flamechurch Pass
    pub fld_mnt_1_2_enter: bool,

    #[default = false]
    /// Exit Eastern Flamechurch Pass
    pub fld_mnt_1_2_exit: bool,

    #[default = false]
    /// Enter Eastern Ku Sands
    pub fld_dst_3_2_enter: bool,

    #[default = false]
    /// Exit Eastern Ku Sands
    pub fld_dst_3_2_exit: bool,

    #[default = false]
    /// Enter Eastern New Delsta Highroad
    pub fld_cty_1_3_enter: bool,

    #[default = false]
    /// Exit Eastern New Delsta Highroad
    pub fld_cty_1_3_exit: bool,

    #[default = false]
    /// Enter Eastern Sai Sands
    pub fld_dst_2_4_enter: bool,

    #[default = false]
    /// Exit Eastern Sai Sands
    pub fld_dst_2_4_exit: bool,

    #[default = false]
    /// Enter Eastern Wellgrove Trail
    pub fld_fst_2_2_enter: bool,

    #[default = false]
    /// Exit Eastern Wellgrove Trail
    pub fld_fst_2_2_exit: bool,

    #[default = false]
    /// Enter Fellsun Ruins
    pub dng_wld_2_2_enter: bool,

    #[default = false]
    /// Exit Fellsun Ruins
    pub dng_wld_2_2_exit: bool,

    #[default = false]
    /// Enter Festival Grounds
    pub fld_fst_1_2_enter: bool,

    #[default = false]
    /// Exit Festival Grounds
    pub fld_fst_1_2_exit: bool,

    #[default = false]
    /// Enter Five-Tiered Tower
    pub dng_dst_3_2_a_enter: bool,

    #[default = false]
    /// Exit Five-Tiered Tower
    pub dng_dst_3_2_a_exit: bool,

    #[default = false]
    /// Enter Five-Tiered Tower: Fourth Floor
    pub dng_dst_3_2_d_enter: bool,

    #[default = false]
    /// Exit Five-Tiered Tower: Fourth Floor
    pub dng_dst_3_2_d_exit: bool,

    #[default = false]
    /// Enter Five-Tiered Tower: Second Floor
    pub dng_dst_3_2_b_enter: bool,

    #[default = false]
    /// Exit Five-Tiered Tower: Second Floor
    pub dng_dst_3_2_b_exit: bool,

    #[default = false]
    /// Enter Five-Tiered Tower: Third Floor
    pub dng_dst_3_2_c_enter: bool,

    #[default = false]
    /// Exit Five-Tiered Tower: Third Floor
    pub dng_dst_3_2_c_exit: bool,

    #[default = false]
    /// Enter Five-Tiered Tower: Top Floor
    pub dng_dst_3_2_e_enter: bool,

    #[default = false]
    /// Exit Five-Tiered Tower: Top Floor
    pub dng_dst_3_2_e_exit: bool,

    #[default = false]
    /// Enter Flamechurch
    pub twn_mnt_1_1_a_enter: bool,

    #[default = false]
    /// Exit Flamechurch
    pub twn_mnt_1_1_a_exit: bool,

    #[default = false]
    /// Enter Flamechurch Pilgrims' Way
    pub fld_mnt_1_1_enter: bool,

    #[default = false]
    /// Exit Flamechurch Pilgrims' Way
    pub fld_mnt_1_1_exit: bool,

    #[default = false]
    /// Enter Flamechurch: Cathedral
    pub twn_mnt_1_2_b_enter: bool,

    #[default = false]
    /// Exit Flamechurch: Cathedral
    pub twn_mnt_1_2_b_exit: bool,

    #[default = false]
    /// Enter Flamechurch: Cathedral Entrance
    pub twn_mnt_1_2_a_enter: bool,

    #[default = false]
    /// Exit Flamechurch: Cathedral Entrance
    pub twn_mnt_1_2_a_exit: bool,

    #[default = false]
    /// Enter Forbidden Shrine
    pub dng_snw_3_1_enter: bool,

    #[default = false]
    /// Exit Forbidden Shrine
    pub dng_snw_3_1_exit: bool,

    #[default = false]
    /// Enter Forest Path
    pub fld_fst_1_1_enter: bool,

    #[default = false]
    /// Exit Forest Path
    pub fld_fst_1_1_exit: bool,

    #[default = false]
    /// Enter Forsaken Graveyard
    pub dng_mnt_2_1_enter: bool,

    #[default = false]
    /// Exit Forsaken Graveyard
    pub dng_mnt_2_1_exit: bool,

    #[default = false]
    /// Enter Frigit Isle: Anchorage
    pub fld_snw_1_1_b_enter: bool,

    #[default = false]
    /// Exit Frigit Isle: Anchorage
    pub fld_snw_1_1_b_exit: bool,

    #[default = false]
    /// Enter Frigit Isle: Entrance
    pub fld_snw_1_1_a_enter: bool,

    #[default = false]
    /// Exit Frigit Isle: Entrance
    pub fld_snw_1_1_a_exit: bool,

    #[default = false]
    /// Enter Frigit Isle: Mining Site
    pub twn_snw_1_1_c_enter: bool,

    #[default = false]
    /// Exit Frigit Isle: Mining Site
    pub twn_snw_1_1_c_exit: bool,

    #[default = false]
    /// Enter Frigit Isle: Prison
    pub twn_snw_1_1_a_enter: bool,

    #[default = false]
    /// Exit Frigit Isle: Prison
    pub twn_snw_1_1_a_exit: bool,

    #[default = false]
    /// Enter Frigit Isle: Yard
    pub twn_snw_1_1_b_enter: bool,

    #[default = false]
    /// Exit Frigit Isle: Yard
    pub twn_snw_1_1_b_exit: bool,

    #[default = false]
    /// Enter Gate of ㌀㌁㌂㌃㌄
    pub dng_ocn_1_4_enter: bool,

    #[default = false]
    /// Exit Gate of ㌀㌁㌂㌃㌄
    pub dng_ocn_1_4_exit: bool,

    #[default = false]
    /// Enter Giff's Manse
    pub dng_wld_1_1_enter: bool,

    #[default = false]
    /// Exit Giff's Manse
    pub dng_wld_1_1_exit: bool,

    #[default = false]
    /// Enter Gravell
    pub twn_wld_3_1_a_enter: bool,

    #[default = false]
    /// Exit Gravell
    pub twn_wld_3_1_a_exit: bool,

    #[default = false]
    /// Enter Guard Outpost
    pub dng_sea_2_2_enter: bool,

    #[default = false]
    /// Exit Guard Outpost
    pub dng_sea_2_2_exit: bool,

    #[default = false]
    /// Enter Healeaks
    pub twn_cty_1_2_b_enter: bool,

    #[default = false]
    /// Exit Healeaks
    pub twn_cty_1_2_b_exit: bool,

    #[default = false]
    /// Enter House Wellows Manor
    pub dng_fst_3_3_enter: bool,

    #[default = false]
    /// Exit House Wellows Manor
    pub dng_fst_3_3_exit: bool,

    #[default = false]
    /// Enter Infernal Castle
    pub dng_snw_3_4_b_enter: bool,

    #[default = false]
    /// Exit Infernal Castle
    pub dng_snw_3_4_b_exit: bool,

    #[default = false]
    /// Enter Ivory Ravine
    pub dng_wld_3_2_enter: bool,

    #[default = false]
    /// Exit Ivory Ravine
    pub dng_wld_3_2_exit: bool,

    #[default = false]
    /// Enter Ku: Castle Town
    pub twn_dst_3_1_a_enter: bool,

    #[default = false]
    /// Enter Ku: Castle Town
    pub twn_dst_3_1_a_fire_enter: bool,

    #[default = false]
    /// Exit Ku: Castle Town
    pub twn_dst_3_1_a_exit: bool,

    #[default = false]
    /// Exit Ku: Castle Town
    pub twn_dst_3_1_a_fire_exit: bool,

    #[default = false]
    /// Enter Lair of the Usurper
    pub dng_sea_1_3_enter: bool,

    #[default = false]
    /// Exit Lair of the Usurper
    pub dng_sea_1_3_exit: bool,

    #[default = false]
    /// Enter Lighthouse Island
    pub fld_ocn_1_2_enter: bool,

    #[default = false]
    /// Exit Lighthouse Island
    pub fld_ocn_1_2_exit: bool,

    #[default = false]
    /// Enter Lostseed
    pub twn_cty_3_1_a_enter: bool,

    #[default = false]
    /// Exit Lostseed
    pub twn_cty_3_1_a_exit: bool,

    #[default = false]
    /// Enter Lostseed Castle
    pub dng_cty_3_2_a_enter: bool,

    #[default = false]
    /// Exit Lostseed Castle
    pub dng_cty_3_2_a_exit: bool,

    #[default = false]
    /// Enter Lostseed Castle: Upper Level
    pub dng_cty_3_2_b_enter: bool,

    #[default = false]
    /// Exit Lostseed Castle: Upper Level
    pub dng_cty_3_2_b_exit: bool,

    #[default = false]
    /// Enter Merry Hills
    pub twn_mnt_3_1_a_enter: bool,

    #[default = false]
    /// Exit Merry Hills
    pub twn_mnt_3_1_a_exit: bool,

    #[default = false]
    /// Enter Merry Hills: Shrine Entrance
    pub twn_mnt_3_1_b_enter: bool,

    #[default = false]
    /// Exit Merry Hills: Shrine Entrance
    pub twn_mnt_3_1_b_exit: bool,

    #[default = false]
    /// Enter Montwise
    pub twn_mnt_2_1_a_enter: bool,

    #[default = false]
    /// Exit Montwise
    pub twn_mnt_2_1_a_exit: bool,

    #[default = false]
    /// Enter Montwise: Library
    pub twn_mnt_2_1_b_enter: bool,

    #[default = false]
    /// Exit Montwise: Library
    pub twn_mnt_2_1_b_exit: bool,

    #[default = false]
    /// Enter Montwise: Underground Arena
    pub twn_mnt_2_1_c_enter: bool,

    #[default = false]
    /// Exit Montwise: Underground Arena
    pub twn_mnt_2_1_c_exit: bool,

    #[default = false]
    /// Enter Moonview Cliff
    pub dng_isd_3_2_d_enter: bool,

    #[default = false]
    /// Exit Moonview Cliff
    pub dng_isd_3_2_d_exit: bool,

    #[default = false]
    /// Enter Mother's Garden
    pub dng_fst_3_2_enter: bool,

    #[default = false]
    /// Exit Mother's Garden
    pub dng_fst_3_2_exit: bool,

    #[default = false]
    /// Enter Mother's Garden: Orphanage
    pub twn_fst_3_2_a_enter: bool,

    #[default = false]
    /// Exit Mother's Garden: Orphanage
    pub twn_fst_3_2_a_exit: bool,

    #[default = false]
    /// Enter Mount Liphia
    pub dng_cty_2_2_enter: bool,

    #[default = false]
    /// Exit Mount Liphia
    pub dng_cty_2_2_exit: bool,

    #[default = false]
    /// Enter Nameless Isle
    pub dng_ocn_1_3_enter: bool,

    #[default = false]
    /// Exit Nameless Isle
    pub dng_ocn_1_3_exit: bool,

    #[default = false]
    /// Enter Nameless Village
    pub twn_isd_3_1_a_enter: bool,

    #[default = false]
    /// Exit Nameless Village
    pub twn_isd_3_1_a_exit: bool,

    #[default = false]
    /// Enter New Delsta
    pub twn_cty_1_1_a_enter: bool,

    #[default = false]
    /// Exit New Delsta
    pub twn_cty_1_1_a_exit: bool,

    #[default = false]
    /// Enter New Delsta Flats
    pub fld_cty_1_2_enter: bool,

    #[default = false]
    /// Exit New Delsta Flats
    pub fld_cty_1_2_exit: bool,

    #[default = false]
    /// Enter New Delsta Harbor: Anchorage
    pub fld_cty_1_1_enter: bool,

    #[default = false]
    /// Exit New Delsta Harbor: Anchorage
    pub fld_cty_1_1_exit: bool,

    #[default = false]
    /// Enter New Delsta: Backstreets
    pub twn_cty_1_1_b_enter: bool,

    #[default = false]
    /// Exit New Delsta: Backstreets
    pub twn_cty_1_1_b_exit: bool,

    #[default = false]
    /// Enter New Delsta: Game Parlor
    pub twn_cty_1_1_c_enter: bool,

    #[default = false]
    /// Exit New Delsta: Game Parlor
    pub twn_cty_1_1_c_exit: bool,

    #[default = false]
    /// Enter North Beasting Traverse
    pub fld_isd_1_2_enter: bool,

    #[default = false]
    /// Exit North Beasting Traverse
    pub fld_isd_1_2_exit: bool,

    #[default = false]
    /// Enter Northern Conning Creek Coast
    pub fld_sea_2_2_enter: bool,

    #[default = false]
    /// Exit Northern Conning Creek Coast
    pub fld_sea_2_2_exit: bool,

    #[default = false]
    /// Enter Northern Montwise Pass
    pub fld_mnt_3_1_enter: bool,

    #[default = false]
    /// Exit Northern Montwise Pass
    pub fld_mnt_3_1_exit: bool,

    #[default = false]
    /// Enter Northern Ryu Sands
    pub fld_dst_1_1_enter: bool,

    #[default = false]
    /// Exit Northern Ryu Sands
    pub fld_dst_1_1_exit: bool,

    #[default = false]
    /// Enter Northern Wellgrove Trail
    pub fld_fst_2_1_enter: bool,

    #[default = false]
    /// Exit Northern Wellgrove Trail
    pub fld_fst_2_1_exit: bool,

    #[default = false]
    /// Enter Ocean ???
    pub fld_ocn_1_4_enter: bool,
    #[default = false]
    /// Exit Ocean ???
    pub fld_ocn_1_4_exit: bool,
    #[default = false]
    /// Enter Old Campsite
    pub fld_dst_2_5_b_enter: bool,

    #[default = false]
    /// Exit Old Campsite
    pub fld_dst_2_5_b_exit: bool,

    #[default = false]
    /// Enter Old Clock Tower
    pub dng_cty_2_3_enter: bool,

    #[default = false]
    /// Exit Old Clock Tower
    pub dng_cty_2_3_exit: bool,

    #[default = false]
    /// Enter On the Water
    pub fld_ocn_1_1_enter: bool,

    #[default = false]
    /// Exit On the Water
    pub fld_ocn_1_1_exit: bool,

    #[default = false]
    /// Enter Oresrush A
    pub twn_wld_1_1_a_enter: bool,

    #[default = false]
    /// Enter Oresrush B
    pub twn_wld_1_1_b_enter: bool,

    #[default = false]
    /// Enter Oresrush C
    pub twn_wld_1_1_c_enter: bool,

    #[default = false]
    /// Exit Oresrush A
    pub twn_wld_1_1_a_exit: bool,

    #[default = false]
    /// Exit Oresrush B
    pub twn_wld_1_1_b_exit: bool,

    #[default = false]
    /// Exit Oresrush C
    pub twn_wld_1_1_c_exit: bool,

    #[default = false]
    /// Enter Oresrush: Foundry
    pub twn_wld_1_2_b_enter: bool,

    #[default = false]
    /// Enter Oresrush: Foundry
    pub twn_wld_1_2_a_enter: bool,

    #[default = false]
    /// Exit Oresrush: Foundry
    pub twn_wld_1_2_b_exit: bool,

    #[default = false]
    /// Exit Oresrush: Foundry
    pub twn_wld_1_2_a_exit: bool,

    #[default = false]
    /// Enter Path to Mount Liphia
    pub fld_cty_2_2_enter: bool,

    #[default = false]
    /// Exit Path to Mount Liphia
    pub fld_cty_2_2_exit: bool,

    #[default = false]
    /// Enter Path to the Bed of the Titan
    pub dng_wld_2_1_a_enter: bool,

    #[default = false]
    /// Exit Path to the Bed of the Titan
    pub dng_wld_2_1_a_exit: bool,

    #[default = false]
    /// Enter Path to the Duskruin Shrine
    pub fld_wld_3_1_enter: bool,

    #[default = false]
    /// Exit Path to the Duskruin Shrine
    pub fld_wld_3_1_exit: bool,

    #[default = false]
    /// Enter Path to the Tombs of the Wardenbeasts
    pub fld_isd_1_1_enter: bool,

    #[default = false]
    /// Exit Path to the Tombs of the Wardenbeasts
    pub fld_isd_1_1_exit: bool,

    #[default = false]
    /// Enter Prison: Underground Passage
    pub dng_snw_1_1_enter: bool,

    #[default = false]
    /// Exit Prison: Underground Passage
    pub dng_snw_1_1_exit: bool,

    #[default = false]
    /// Enter Quicksand Gaol
    pub dng_dst_2_2_enter: bool,

    #[default = false]
    /// Exit Quicksand Gaol
    pub dng_dst_2_2_exit: bool,

    #[default = false]
    /// Enter Rifted Rock
    pub dng_isd_3_1_c_enter: bool,

    #[default = false]
    /// Exit Rifted Rock
    pub dng_isd_3_1_c_exit: bool,

    #[default = false]
    /// Enter Road to Mother's Garden
    pub fld_fst_3_1_enter: bool,

    #[default = false]
    /// Exit Road to Mother's Garden
    pub fld_fst_3_1_exit: bool,

    #[default = false]
    /// Enter Roque Island
    pub twn_sea_3_1_a_enter: bool,

    #[default = false]
    /// Exit Roque Island
    pub twn_sea_3_1_a_exit: bool,

    #[default = false]
    /// Enter Roque Island: Anchorage
    pub fld_sea_3_1_enter: bool,

    #[default = false]
    /// Exit Roque Island: Anchorage
    pub fld_sea_3_1_exit: bool,

    #[default = false]
    /// Enter Roque Island: Headquarters
    pub twn_sea_3_1_b_enter: bool,

    #[default = false]
    /// Exit Roque Island: Headquarters
    pub twn_sea_3_1_b_exit: bool,

    #[default = false]
    /// Enter Ruffians' Hideout
    pub dng_snw_1_2_enter: bool,

    #[default = false]
    /// Exit Ruffians' Hideout
    pub dng_snw_1_2_exit: bool,

    #[default = false]
    /// Enter Ryu
    pub twn_dst_1_1_a_enter: bool,

    #[default = false]
    /// Exit Ryu
    pub twn_dst_1_1_a_exit: bool,

    #[default = false]
    /// Enter Sacred Guard Ship
    pub dng_sea_1_2_enter: bool,

    #[default = false]
    /// Exit Sacred Guard Ship
    pub dng_sea_1_2_exit: bool,

    #[default = false]
    /// Enter Sacred Peak Altahe
    pub dng_snw_3_3_enter: bool,

    #[default = false]
    /// Exit Sacred Peak Altahe
    pub dng_snw_3_3_exit: bool,

    #[default = false]
    /// Enter Sai
    pub twn_dst_2_1_a_enter: bool,

    #[default = false]
    /// Exit Sai
    pub twn_dst_2_1_a_exit: bool,

    #[default = false]
    /// Enter Sai: East District
    pub twn_dst_2_1_b_enter: bool,

    #[default = false]
    /// Exit Sai: East District
    pub twn_dst_2_1_b_exit: bool,

    #[default = false]
    /// Enter Sand Lion's Den
    pub dng_dst_2_4_enter: bool,

    #[default = false]
    /// Exit Sand Lion's Den
    pub dng_dst_2_4_exit: bool,

    #[default = false]
    /// Enter Sandflow Pass
    pub fld_dst_2_5_a_enter: bool,

    #[default = false]
    /// Exit Sandflow Pass
    pub fld_dst_2_5_a_exit: bool,

    #[default = false]
    /// Enter Seat of the Water Sprite
    pub dng_mnt_2_2_enter: bool,

    #[default = false]
    /// Exit Seat of the Water Sprite
    pub dng_mnt_2_2_exit: bool,

    #[default = false]
    /// Enter Secret Forest
    pub dng_fst_2_1_enter: bool,

    #[default = false]
    /// Exit Secret Forest
    pub dng_fst_2_1_exit: bool,

    #[default = false]
    /// Enter Shipwreck of the Empress
    pub dng_ocn_1_1_enter: bool,

    #[default = false]
    /// Exit Shipwreck of the Empress
    pub dng_ocn_1_1_exit: bool,

    #[default = false]
    /// Enter Shrine of Ul'sterra
    pub dng_mnt_3_1_a_enter: bool,

    #[default = false]
    /// Exit Shrine of Ul'sterra
    pub dng_mnt_3_1_a_exit: bool,

    #[default = false]
    /// Enter Silver Mine
    pub fld_wld_1_1_a_enter: bool,

    #[default = false]
    /// Exit Silver Mine
    pub fld_wld_1_1_a_exit: bool,

    #[default = false]
    /// Enter Sinking Ruins
    pub dng_isd_2_1_enter: bool,

    #[default = false]
    /// Exit Sinking Ruins
    pub dng_isd_2_1_exit: bool,

    #[default = false]
    /// Enter Snowhares' Den
    pub dng_snw_2_1_enter: bool,

    #[default = false]
    /// Exit Snowhares' Den
    pub dng_snw_2_1_exit: bool,

    #[default = false]
    /// Enter Southern Cape Cold Snows
    pub fld_snw_1_2_enter: bool,

    #[default = false]
    /// Exit Southern Cape Cold Snows
    pub fld_snw_1_2_exit: bool,

    #[default = false]
    /// Enter Southern Clockbank Highroad
    pub fld_cty_2_1_enter: bool,

    #[default = false]
    /// Exit Southern Clockbank Highroad
    pub fld_cty_2_1_exit: bool,

    #[default = false]
    /// Enter Southern Crackridge Wilds
    pub fld_wld_2_2_enter: bool,

    #[default = false]
    /// Exit Southern Crackridge Wilds
    pub fld_wld_2_2_exit: bool,

    #[default = false]
    /// Enter Southern Cropdale Trail
    pub fld_fst_1_4_enter: bool,

    #[default = false]
    /// Exit Southern Cropdale Trail
    pub fld_fst_1_4_exit: bool,

    #[default = false]
    /// Enter Southern Ku Sands
    pub fld_dst_3_1_b_enter: bool,

    #[default = false]
    /// Exit Southern Ku Sands
    pub fld_dst_3_1_b_exit: bool,

    #[default = false]
    /// Enter Southern Nameless Village Traverse
    pub fld_isd_3_3_enter: bool,

    #[default = false]
    /// Exit Southern Nameless Village Traverse
    pub fld_isd_3_3_exit: bool,

    #[default = false]
    /// Enter Southern Oresrush Wilds
    pub fld_wld_1_3_enter: bool,

    #[default = false]
    /// Exit Southern Oresrush Wilds
    pub fld_wld_1_3_exit: bool,

    #[default = false]
    /// Enter Southern Sai Sands
    pub fld_dst_2_3_enter: bool,

    #[default = false]
    /// Exit Southern Sai Sands
    pub fld_dst_2_3_exit: bool,

    #[default = false]
    /// Enter Southern Stormhail Snows
    pub fld_snw_3_1_enter: bool,

    #[default = false]
    /// Exit Southern Stormhail Snows
    pub fld_snw_3_1_exit: bool,

    #[default = false]
    /// Enter Southern Timberain Trail
    pub fld_fst_3_2_enter: bool,

    #[default = false]
    /// Exit Southern Timberain Trail
    pub fld_fst_3_2_exit: bool,

    #[default = false]
    /// Enter Stage of the Moon and Sun
    pub dng_mnt_3_1_b_enter: bool,

    #[default = false]
    /// Exit Stage of the Moon and Sun
    pub dng_mnt_3_1_b_exit: bool,

    #[default = false]
    /// Enter Starfall Spring
    pub dng_fst_1_2_enter: bool,

    #[default = false]
    /// Exit Starfall Spring
    pub dng_fst_1_2_exit: bool,

    #[default = false]
    /// Enter Stormhail
    pub twn_snw_3_1_a_enter: bool,

    #[default = false]
    /// Exit Stormhail
    pub twn_snw_3_1_a_exit: bool,

    #[default = false]
    /// Enter Stormhail: Bridge
    pub twn_snw_3_1_c_enter: bool,

    #[default = false]
    /// Exit Stormhail: Bridge
    pub twn_snw_3_1_c_exit: bool,

    #[default = false]
    /// Enter Stormhail: Castle Mei
    pub twn_snw_3_2_a_enter: bool,

    #[default = false]
    /// Exit Stormhail: Castle Mei
    pub twn_snw_3_2_a_exit: bool,

    #[default = false]
    /// Enter Stormhail: Sacred Guard Headquarters
    pub twn_snw_3_1_b_enter: bool,

    #[default = false]
    /// Exit Stormhail: Sacred Guard Headquarters
    pub twn_snw_3_1_b_exit: bool,

    #[default = false]
    /// Enter Stormy Cape
    pub dng_isd_3_2_c_enter: bool,

    #[default = false]
    /// Exit Stormy Cape
    pub dng_isd_3_2_c_exit: bool,

    #[default = false]
    /// Enter Summit of Strife
    pub dng_isd_3_2_a_enter: bool,

    #[default = false]
    /// Exit Summit of Strife
    pub dng_isd_3_2_a_exit: bool,

    #[default = false]
    /// Enter Sunken Maw
    pub dng_cty_1_4_enter: bool,

    #[default = false]
    /// Exit Sunken Maw
    pub dng_cty_1_4_exit: bool,

    #[default = false]
    /// Enter The Lost Isle
    pub fld_ocn_1_3_enter: bool,

    #[default = false]
    /// Exit The Lost Isle
    pub fld_ocn_1_3_exit: bool,

    #[default = false]
    /// Enter The Roque Company: East Tower
    pub dng_sea_3_1_b_enter: bool,

    #[default = false]
    /// Exit The Roque Company: East Tower
    pub dng_sea_3_1_b_exit: bool,

    #[default = false]
    /// Enter The Roque Company: Factory
    pub dng_cty_2_1_enter: bool,

    #[default = false]
    /// Exit The Roque Company: Factory
    pub dng_cty_2_1_exit: bool,

    #[default = false]
    /// Enter The Roque Company: West Tower
    pub dng_sea_3_1_a_enter: bool,

    #[default = false]
    /// Exit The Roque Company: West Tower
    pub dng_sea_3_1_a_exit: bool,

    #[default = false]
    /// Enter Theater
    pub dng_cty_1_2_enter: bool,

    #[default = false]
    /// Exit Theater
    pub dng_cty_1_2_exit: bool,

    #[default = false]
    /// Enter Timberain
    pub twn_fst_3_1_a_enter: bool,

    #[default = false]
    /// Exit Timberain
    pub twn_fst_3_1_a_exit: bool,

    #[default = false]
    /// Enter Timberain Castle
    pub dng_fst_3_1_a_enter: bool,

    #[default = false]
    /// Exit Timberain Castle
    pub dng_fst_3_1_a_exit: bool,

    #[default = false]
    /// Enter Timberain Castle: Roof
    pub dng_fst_3_1_b_enter: bool,

    #[default = false]
    /// Exit Timberain Castle: Roof
    pub dng_fst_3_1_b_exit: bool,

    #[default = false]
    /// Enter Timberain Castle: Town Square
    pub twn_fst_3_1_b_enter: bool,

    #[default = false]
    /// Exit Timberain Castle: Town Square
    pub twn_fst_3_1_b_exit: bool,

    #[default = false]
    /// Enter Tombs of the Wardenbeasts
    pub dng_isd_1_1_enter: bool,

    #[default = false]
    /// Exit Tombs of the Wardenbeasts
    pub dng_isd_1_1_exit: bool,

    #[default = false]
    /// Enter Tranquil Grotto
    pub dng_dst_3_1_enter: bool,

    #[default = false]
    /// Exit Tranquil Grotto
    pub dng_dst_3_1_exit: bool,

    #[default = false]
    /// Enter Tropu'hopu
    pub twn_isd_2_1_a_enter: bool,

    #[default = false]
    /// Exit Tropu'hopu
    pub twn_isd_2_1_a_exit: bool,

    #[default = false]
    /// Enter Tropu'hopu: Floating Theater
    pub twn_isd_2_1_b_enter: bool,

    #[default = false]
    /// Exit Tropu'hopu: Floating Theater
    pub twn_isd_2_1_b_exit: bool,

    #[default = false]
    /// Enter Tropu'hopu: Shipyard
    pub twn_isd_2_1_c_enter: bool,

    #[default = false]
    /// Exit Tropu'hopu: Shipyard
    pub twn_isd_2_1_c_exit: bool,

    #[default = false]
    /// Enter Underground Laboratory
    pub dng_mnt_2_3_enter: bool,

    #[default = false]
    /// Exit Underground Laboratory
    pub dng_mnt_2_3_exit: bool,

    #[default = false]
    /// Enter Underground Waterway
    pub dng_cty_3_1_enter: bool,

    #[default = false]
    /// Exit Underground Waterway
    pub dng_cty_3_1_exit: bool,

    #[default = false]
    /// Enter Unfinished Tunnel
    pub dng_wld_1_2_enter: bool,

    #[default = false]
    /// Exit Unfinished Tunnel
    pub dng_wld_1_2_exit: bool,

    #[default = false]
    /// Enter unknown
    pub evt_end_30_001_enter: bool,

    #[default = false]
    /// Enter Veil of Trees
    pub dng_fst_1_1_enter: bool,

    #[default = false]
    /// Exit Veil of Trees
    pub dng_fst_1_1_exit: bool,

    #[default = false]
    /// Enter Verdant Wood
    pub dng_isd_3_2_b_enter: bool,

    #[default = false]
    /// Exit Verdant Wood
    pub dng_isd_3_2_b_exit: bool,

    #[default = false]
    /// Enter Vidania
    pub fld_atl_3_1_enter: bool,

    #[default = false]
    /// Exit Vidania
    pub fld_atl_3_1_exit: bool,

    #[default = false]
    /// Enter Wandering Wood A
    pub dng_isd_3_1_a_enter: bool,

    #[default = false]
    /// Enter Wandering Wood B
    pub dng_isd_3_1_b_enter: bool,

    #[default = false]
    /// Exit Wandering Wood A
    pub dng_isd_3_1_a_exit: bool,

    #[default = false]
    /// Exit Wandering Wood B
    pub dng_isd_3_1_b_exit: bool,

    #[default = false]
    /// Enter Wellgrove
    pub twn_fst_2_1_a_enter: bool,

    #[default = false]
    /// Exit Wellgrove
    pub twn_fst_2_1_a_exit: bool,

    #[default = false]
    /// Enter Wellgrove: Alrond's Estate
    pub twn_fst_2_1_b_enter: bool,

    #[default = false]
    /// Exit Wellgrove: Alrond's Estate
    pub twn_fst_2_1_b_exit: bool,

    #[default = false]
    /// Enter Western Canalbrine Coast
    pub fld_sea_1_3_enter: bool,

    #[default = false]
    /// Exit Western Canalbrine Coast
    pub fld_sea_1_3_exit: bool,

    #[default = false]
    /// Enter Western Clockbank Highroad
    pub fld_cty_1_4_enter: bool,

    #[default = false]
    /// Exit Western Clockbank Highroad
    pub fld_cty_1_4_exit: bool,

    #[default = false]
    /// Enter Western Conning Creek Coast
    pub fld_sea_2_3_enter: bool,

    #[default = false]
    /// Exit Western Conning Creek Coast
    pub fld_sea_2_3_exit: bool,

    #[default = false]
    /// Enter Western Crackridge Wilds
    pub fld_wld_2_1_enter: bool,

    #[default = false]
    /// Exit Western Crackridge Wilds
    pub fld_wld_2_1_exit: bool,

    #[default = false]
    /// Enter Western Gravell Wilds
    pub fld_wld_3_2_enter: bool,

    #[default = false]
    /// Exit Western Gravell Wilds
    pub fld_wld_3_2_exit: bool,

    #[default = false]
    /// Enter Western Merry Hills Pass
    pub fld_mnt_3_2_enter: bool,

    #[default = false]
    /// Exit Western Merry Hills Pass
    pub fld_mnt_3_2_exit: bool,

    #[default = false]
    /// Enter Western Montwise Pass
    pub fld_mnt_2_1_enter: bool,

    #[default = false]
    /// Exit Western Montwise Pass
    pub fld_mnt_2_1_exit: bool,

    #[default = false]
    /// Enter Western Sai Sands
    pub fld_dst_2_2_enter: bool,

    #[default = false]
    /// Exit Western Sai Sands
    pub fld_dst_2_2_exit: bool,

    #[default = false]
    /// Enter Western Tropu'hopu Traverse
    pub fld_isd_2_1_enter: bool,

    #[default = false]
    /// Exit Western Tropu'hopu Traverse
    pub fld_isd_2_1_exit: bool,

    #[default = false]
    /// Enter Western Winterbloom Snows
    pub fld_snw_2_2_enter: bool,

    #[default = false]
    /// Exit Western Winterbloom Snows
    pub fld_snw_2_2_exit: bool,

    #[default = false]
    /// Enter Winterbloom
    pub twn_snw_2_1_a_enter: bool,

    #[default = false]
    /// Exit Winterbloom
    pub twn_snw_2_1_a_exit: bool,

    #[default = false]
    /// Enter Winterbloom: Thieves' Quarter
    pub twn_snw_2_1_b_enter: bool,

    #[default = false]
    /// Exit Winterbloom: Thieves' Quarter
    pub twn_snw_2_1_b_exit: bool,
}
