#[derive(asr::Settings)]
pub struct Settings {
    /// Load/Autosave Remover
    pub load_removal: bool,

    /// Job License - Inventor
    pub job_license_inventor: bool,

    /// Job License - Armsmaster
    pub job_license_armsmaster: bool,

    /// Job License - Hunter
    pub job_license_hunter: bool,

    /// Job License - Thief
    pub job_license_thief: bool,

    /// Job License - Cleric
    pub job_license_cleric: bool,

    /// Job License - Scholar
    pub job_license_scholar: bool,

    /// Job License - Merchant
    pub job_license_merchant: bool,

    /// Job License - Dancer
    pub job_license_dancer: bool,

    /// Job License - Warrior
    pub job_license_warrior: bool,

    /// Job License - Apothecary
    pub job_license_apothecary: bool,

    /// Throne Main Story Complete (End Single Story)
    pub throne_story_complete: bool,

    /// Throne joins the party
    pub throne_joins: bool,

    /// Throne 1 - On the Run
    pub throne_10: bool,

    /// Throne 1 - A Locked Door
    pub throne_20: bool,

    /// Throne 1 - A Traitor Among Us
    pub throne_30: bool,

    /// Throne 1 - Silencing the Guard
    pub throne_40: bool,

    /// Throne 1 - Meaningless Bloodshed
    pub throne_50: bool,

    /// Throne 1 - Enemies Lying in Wait
    pub throne_60: bool,

    /// Throne 1 - To the World Above
    pub throne_70: bool,

    /// Throne 1 - The Blacksnakes
    pub throne_80: bool,

    /// Throne 1 - Father
    pub throne_90: bool,

    /// Throne 1 - Mother
    pub throne_100: bool,

    /// Throne 1 - My Dream
    pub throne_110: bool,

    /// Throne 1 - Scaracci the Traitor
    pub throne_120: bool,

    /// Throne 1 - Diamante's Estate
    pub throne_150: bool,

    /// Throne 1 - Successful Infiltration
    pub throne_160: bool,

    /// Throne 1 - Selecting a Successor
    pub throne_180: bool,

    /// Throne 1 - Freedom
    pub throne_190: bool,

    /// Throne 1 - Throné's Resolve
    pub throne_500: bool,

    /// Throne 2A - Pursuing Mother
    pub throne_510: bool,

    /// Throne 2A - Feigning Ignorance
    pub throne_520: bool,

    /// Throne 2A - A Horse
    pub throne_530: bool,

    /// Throne 2A - Information on Mother
    pub throne_540: bool,

    /// Throne 2A - Followed
    pub throne_550: bool,

    /// Throne 2A - The Pursuer's Face
    pub throne_560: bool,

    /// Throne 2A - The Masked Boy
    pub throne_570: bool,

    /// Throne 2A - To the Old Foundry
    pub throne_580: bool,

    /// Throne 2A - The Slaver
    pub throne_590: bool,

    /// Throne 2A - Death's Table
    pub throne_610: bool,

    /// Throne 2A - The Boy's True Face
    pub throne_1000: bool,

    /// Throne 2B - Pursuing Father
    pub throne_1010: bool,

    /// Throne 2B - Trouble at the Tavern
    pub throne_1020: bool,

    /// Throne 2B - Father Appears
    pub throne_1030: bool,

    /// Throne 2B - Visiting the Snowhares
    pub throne_1040: bool,

    /// Throne 2B - Hesitation
    pub throne_1050: bool,

    /// Throne 2B - First Job
    pub throne_1060: bool,

    /// Throne 2B - Tainted with the Stench of Blood
    pub throne_1070: bool,

    /// Throne 2B - At the Snowhares' Den
    pub throne_1080: bool,

    /// Throne 2B - Bergomi
    pub throne_1090: bool,

    /// Throne 2B - True Intentions
    pub throne_1100: bool,

    /// Throne 2B - If I Am to Be Free
    pub throne_1500: bool,

    /// Throne 3A - Looking the Part
    pub throne_1510: bool,

    /// Throne 3A - A Young Thief
    pub throne_1520: bool,

    /// Throne 3A - The Bell Chimes
    pub throne_1530: bool,

    /// Throne 3A - A True Snake
    pub throne_1540: bool,

    /// Throne 3A - Subordination
    pub throne_1550: bool,

    /// Throne 3A - Mother Must Die
    pub throne_1560: bool,

    /// Throne 3A - Key to the Door
    pub throne_1570: bool,

    /// Throne 3A - Clash with Mother
    pub throne_1590: bool,

    /// Throne 3A - The New Leader
    pub throne_1600: bool,

    /// Throne 3A - Mother's Final Moments
    pub throne_1610: bool,

    /// Throne 3A - Daughter of Snakes
    pub throne_2000: bool,

    /// Throne 3B - Where Father Awaits
    pub throne_2010: bool,

    /// Throne 3B - Destiny
    pub throne_2020: bool,

    /// Throne 3B - Marietta
    pub throne_2030: bool,

    /// Throne 3B - The Abandoned Church
    pub throne_2040: bool,

    /// Throne 3B - That Which Cannot Be Stolen
    pub throne_2050: bool,

    /// Throne 3B - Baby
    pub throne_2060: bool,

    /// Throne 3B - Clash with Father
    pub throne_2070: bool,

    /// Throne 3B - A Father and Daughter
    pub throne_2080: bool,

    /// Throne 3B - Dad
    pub throne_2090: bool,

    /// Throne 3B - A Heart Unfulfilled
    pub throne_2500: bool,

    /// Throne 4 - Orphaned Snakes
    pub throne_2510: bool,

    /// Throne 4 - The Scent of Cigarettes
    pub throne_2520: bool,

    /// Throne 4 - The Blacksnakes' Cemetery
    pub throne_2530: bool,

    /// Throne 4 - Engraved on the Tomb
    pub throne_2540: bool,

    /// Throne 4 - Where It All Began
    pub throne_2550: bool,

    /// Throne 4 - A Door Unlocked
    pub throne_2560: bool,

    /// Throne 4 - A Ropeway Home
    pub throne_2580: bool,

    /// Throne 4 - In the Gondola
    pub throne_2600: bool,

    /// Throne 4 - To Lostseed
    pub throne_2610: bool,

    /// Throne 4 - A Baby's Cries
    pub throne_2620: bool,

    /// Throne 4 - An Old Castle
    pub throne_2630: bool,

    /// Throne 4 - Claude, the Father
    pub throne_2640: bool,

    /// Throne 4 - His Greatest Masterpiece
    pub throne_2650: bool,

    /// Throne 4 - A Collar Removed
    pub throne_2660: bool,

    /// Throne 4 - Throné
    pub throne_2670: bool,

    /// Throne 4 - The Stench of Blood
    pub throne_3000: bool,

    /// Temenos Main Story Complete (End Single Story)
    pub temenos_story_complete: bool,

    /// Temenos joins the party
    pub temenos_joins: bool,

    /// Temenos 1 - The Eight Gods
    pub temenos_10: bool,

    /// Temenos 1 - The Wicked God, Vide
    pub temenos_20: bool,

    /// Temenos 1 - Temenos and the Children
    pub temenos_30: bool,

    /// Temenos 1 - Temenos and the Pontiff
    pub temenos_40: bool,

    /// Temenos 1 - Crick, Newly Anointed Knight
    pub temenos_60: bool,

    /// Temenos 1 - Godsblade
    pub temenos_90: bool,

    /// Temenos 1 - The Locked Cathedral
    pub temenos_100: bool,

    /// Temenos 1 - An Unconventional Inquisitor
    pub temenos_110: bool,

    /// Temenos 1 - The Pontiff's Death
    pub temenos_120: bool,

    /// Temenos 1 - Suspected Foul Play
    pub temenos_130: bool,

    /// Temenos 1 - The Sacred Guard Arrives
    pub temenos_140: bool,

    /// Temenos 1 - The Pontiff's Funeral
    pub temenos_150: bool,

    /// Temenos 1 - A Note Between the Pages
    pub temenos_160: bool,

    /// Temenos 1 - Reassigned
    pub temenos_170: bool,

    /// Temenos 1 - In Search of Truth
    pub temenos_500: bool,

    /// Temenos 2 - The Second Victim
    pub temenos_510: bool,

    /// Temenos 2 - Lucian the Theologian
    pub temenos_520: bool,

    /// Temenos 2 - Disturbance in Canalbrine
    pub temenos_530: bool,

    /// Temenos 2 - Reunited with Crick
    pub temenos_540: bool,

    /// Temenos 2 - Lucian the Culprit?
    pub temenos_550: bool,

    /// Temenos 2 - A Forceful Entry
    pub temenos_560: bool,

    /// Temenos 2 - The Third Victim
    pub temenos_570: bool,

    /// Temenos 2 - Captain Kaldena
    pub temenos_580: bool,

    /// Temenos 2 - The Search for Evidence
    pub temenos_590: bool,

    /// Temenos 2 - Clue: Note
    pub temenos_600: bool,

    /// Temenos 2 - Clue: Book
    pub temenos_610: bool,

    /// Temenos 2 - Clue: Memo
    pub temenos_620: bool,

    /// Temenos 2 - Dancer at the Tavern
    pub temenos_630: bool,

    /// Temenos 2 - The Killer Is Among Us
    pub temenos_640: bool,

    /// Temenos 2 - The Culprit Is You!
    pub temenos_650: bool,

    /// Temenos 2 - The Fleeing Vados
    pub temenos_660: bool,

    /// Temenos 2 - Time for Answers
    pub temenos_670: bool,

    /// Temenos 2 - The Sacred Guard Interferes
    pub temenos_680: bool,

    /// Temenos 2 - Kaldena and Crick
    pub temenos_690: bool,

    /// Temenos 2 - Ort the Sanctum Knight
    pub temenos_700: bool,

    /// Temenos 2 - Two Paths
    pub temenos_1000: bool,

    /// Temenos 3A - Before the Inquisition
    pub temenos_1010: bool,

    /// Temenos 3A - Mindt's Letter
    pub temenos_1020: bool,

    /// Temenos 3A - To the Sacred Guard Headquarters
    pub temenos_1030: bool,

    /// Temenos 3A - A Shocking Reunion
    pub temenos_1040: bool,

    /// Temenos 3A - No Record of Arrival
    pub temenos_1050: bool,

    /// Temenos 3A - Mysterious Meddler
    pub temenos_1060: bool,

    /// Temenos 3A - Vados the Corpse
    pub temenos_1070: bool,

    /// Temenos 3A - Bring the Truth to Light
    pub temenos_1080: bool,

    /// Temenos 3A - In Search of Evidence, Alone
    pub temenos_1100: bool,

    /// Temenos 3A - Crick in the Archives
    pub temenos_1110: bool,

    /// Temenos 3A - Temenos at the Inn
    pub temenos_1120: bool,

    /// Temenos 3A - An Omen
    pub temenos_1130: bool,

    /// Temenos 3A - Crick's Death
    pub temenos_1140: bool,

    /// Temenos 3A - Crick's Clue: Words in the Shelf
    pub temenos_1170: bool,

    /// Temenos 3A - Crick's Clue: Mysterious Mechanism
    pub temenos_1190: bool,

    /// Temenos 3A - A Hidden Passageway
    pub temenos_1200: bool,

    /// Temenos 3A - The Book of Night
    pub temenos_1210: bool,

    /// Temenos 3A - Crick's Murderer
    pub temenos_1220: bool,

    /// Temenos 3A - The Mastermind
    pub temenos_1230: bool,

    /// Temenos 3A - Kaldena's Plans
    pub temenos_1240: bool,

    /// Temenos 3A - Temenos's Resolve
    pub temenos_1250: bool,

    /// Temenos 3A - To End Kaldena's Treachery
    pub temenos_1500: bool,

    /// Temenos 3B - The Fellsun Ruins
    pub temenos_1510: bool,

    /// Temenos 3B - Renouncers of the Sacred Flame
    pub temenos_1530: bool,

    /// Temenos 3B - Roi
    pub temenos_1540: bool,

    /// Temenos 3B - The Investigation Begins
    pub temenos_1550: bool,

    /// Temenos 3B - Guiding a Troubled Traveler
    pub temenos_1560: bool,

    /// Temenos 3B - Reiza's Purpose
    pub temenos_1580: bool,

    /// Temenos 3B - The Kal Ruins
    pub temenos_1590: bool,

    /// Temenos 3B - Mysterious Mural
    pub temenos_1600: bool,

    /// Temenos 3B - Horror in the Ruins
    pub temenos_1610: bool,

    /// Temenos 3B - Rite of the Night
    pub temenos_1620: bool,

    /// Temenos 3B - Sins of the Moonshade Order
    pub temenos_1630: bool,

    /// Temenos 3B - Uncovering the Truth
    pub temenos_2000: bool,

    /// Temenos 4 - Memories of a Massacre
    pub temenos_2010: bool,

    /// Temenos 4 - Kaldena's Resolve
    pub temenos_2020: bool,

    /// Temenos 4 - The Path to the Rite
    pub temenos_2030: bool,

    /// Temenos 4 - The Nameless Village
    pub temenos_2040: bool,

    /// Temenos 4 - Passing the Trial
    pub temenos_2050: bool,

    /// Temenos 4 - Guardian Beastling
    pub temenos_2060: bool,

    /// Temenos 4 - Kaldena's Rite
    pub temenos_2070: bool,

    /// Temenos 4 - The Fallen Godsblades
    pub temenos_2080: bool,

    /// Temenos 4 - The Inquisition
    pub temenos_2090: bool,

    /// Temenos 4 - Swallowed by Darkness
    pub temenos_2100: bool,

    /// Temenos 4 - Case Closed
    pub temenos_2110: bool,

    /// Temenos 4 - A Journey Ends, a Stroll Begins
    pub temenos_2500: bool,

    /// Partitio Main Story Complete (End Single Story)
    pub partitio_story_complete: bool,

    /// Partitio joins the party
    pub partitio_joins: bool,

    /// Partitio 1 - There Ain't Scratch
    pub partitio_10: bool,

    /// Partitio 1 - A Silver Town
    pub partitio_40: bool,

    /// Partitio 1 - Mischief at the Mine
    pub partitio_60: bool,

    /// Partitio 1 - The Miners and the Gang
    pub partitio_90: bool,

    /// Partitio 1 - The Gang Defeated
    pub partitio_100: bool,

    /// Partitio 1 - The Partners Part Ways
    pub partitio_110: bool,

    /// Partitio 1 - Your Gaze Is on the Horizon
    pub partitio_120: bool,

    /// Partitio 1 - Bedridden Pops
    pub partitio_130: bool,

    /// Partitio 1 - Poverty
    pub partitio_150: bool,

    /// Partitio 1 - A Word of Advice
    pub partitio_170: bool,

    /// Partitio 1 - Take Back the Town
    pub partitio_180: bool,

    /// Partitio 1 - Contract Rewritten
    pub partitio_190: bool,

    /// Partitio 1 - The Landowner's True Identity
    pub partitio_200: bool,

    /// Partitio 1 - The Devil Called Poverty
    pub partitio_500: bool,

    /// Partitio 2 - The Eastern Continent
    pub partitio_510: bool,

    /// Partitio 2 - Heavy Taxes
    pub partitio_520: bool,

    /// Partitio 2 - Ori the Scrivener
    pub partitio_530: bool,

    /// Partitio 2 - Lost in Thought
    pub partitio_540: bool,

    /// Partitio 2 - Those Worthy of Wealth
    pub partitio_550: bool,

    /// Partitio 2 - President Not on the Premises
    pub partitio_560: bool,

    /// Partitio 2 - Floyd the Engineer
    pub partitio_580: bool,

    /// Partitio 2 - Boiler Material
    pub partitio_600: bool,

    /// Partitio 2 - Clockite
    pub partitio_610: bool,

    /// Partitio 2 - A Clockmaker's Art
    pub partitio_620: bool,

    /// Partitio 2 - New Steam Engine, Complete!
    pub partitio_630: bool,

    /// Partitio 2 - A Celebration of Success
    pub partitio_640: bool,

    /// Partitio 2 - New Steam Engine, Gone!
    pub partitio_650: bool,

    /// Partitio 2 - Garnet, Roque's Beloved Hound
    pub partitio_670: bool,

    /// Partitio 2 - An 80-Billion-Leaf Deal
    pub partitio_680: bool,

    /// Partitio 2 - The Richest Noble
    pub partitio_690: bool,

    /// Partitio 2 - A Promise to Shoeshine Will
    pub partitio_1000: bool,

    /// Partitio 3 - Home of the Richest Noble
    pub partitio_1010: bool,

    /// Partitio 3 - A Bad Deal Gone Good
    pub partitio_1020: bool,

    /// Partitio 3 - Winning an Audience with Alrond
    pub partitio_1030: bool,

    /// Partitio 3 - A Distinguished Achievement
    pub partitio_1040: bool,

    /// Partitio 3 - Wealthy Noble Alrond
    pub partitio_1050: bool,

    /// Partitio 3 - Inspecting the Town
    pub partitio_1060: bool,

    /// Partitio 3 - The Ideal Building
    pub partitio_1070: bool,

    /// Partitio 3 - Preparing to Open Shop
    pub partitio_1080: bool,

    /// Partitio 3 - Procuring the Goods
    pub partitio_1090: bool,

    /// Partitio 3 - Alrond's Department Store
    pub partitio_1130: bool,

    /// Partitio 3 - White Fog
    pub partitio_1140: bool,

    /// Partitio 3 - Manse in the Mist
    pub partitio_1150: bool,

    /// Partitio 3 - Hidden in the Fog
    pub partitio_1160: bool,

    /// Partitio 3 - Thurston's Loss
    pub partitio_1170: bool,

    /// Partitio 3 - 80 Billion to Roque Island
    pub partitio_1500: bool,

    /// Partitio 4 - Once-in-a-Lifetime Deal
    pub partitio_1510: bool,

    /// Partitio 4 - Roque's Steam Engine
    pub partitio_1520: bool,

    /// Partitio 4 - Ori's Big Scoop
    pub partitio_1530: bool,

    /// Partitio 4 - Never Underestimate a Scrivener
    pub partitio_1540: bool,

    /// Partitio 4 - Don't Look Back!
    pub partitio_1550: bool,

    /// Partitio 4 - The Conference Begins
    pub partitio_1560: bool,

    /// Partitio 4 - Hold Your Horses!
    pub partitio_1580: bool,

    /// Partitio 4 - The 80-Billion-Leaf Man
    pub partitio_1590: bool,

    /// Partitio 4 - A Done Deal
    pub partitio_1600: bool,

    /// Partitio 4 - Steam Tank Obsidian
    pub partitio_1610: bool,

    /// Partitio 4 - His True Desire
    pub partitio_1620: bool,

    /// Partitio 4 - Hiring Roque
    pub partitio_1630: bool,

    /// Partitio 4 - Partitio & Roque
    pub partitio_1640: bool,

    /// Partitio 4 - The Industrial Revolution
    pub partitio_1650: bool,

    /// Partitio 4 - Traveling the World
    pub partitio_2000: bool,

    /// Osvald Main Story Complete (End Single Story)
    pub osvald_story_complete: bool,

    /// Osvald joins the party
    pub osvald_joins: bool,

    /// Osvald 1 - The Trial
    pub osvald_10: bool,

    /// Osvald 1 - Frigit Isle
    pub osvald_20: bool,

    /// Osvald 1 - Osvald's Notebook
    pub osvald_30: bool,

    /// Osvald 1 - Warden Davids
    pub osvald_40: bool,

    /// Osvald 1 - The Escape Plan
    pub osvald_50: bool,

    /// Osvald 1 - Forced Labor
    pub osvald_60: bool,

    /// Osvald 1 - Three Pieces
    pub osvald_70: bool,

    /// Osvald 1 - Another Dream
    pub osvald_80: bool,

    /// Osvald 1 - Researching the One True Magic
    pub osvald_90: bool,

    /// Osvald 1 - Awakening
    pub osvald_100: bool,

    /// Osvald 1 - A Third Dream
    pub osvald_120: bool,

    /// Osvald 1 - Harvey's Scheme
    pub osvald_130: bool,

    /// Osvald 1 - A Rude Awakening
    pub osvald_140: bool,

    /// Osvald 1 - Report on the Wares
    pub osvald_160: bool,

    /// Osvald 1 - The Time Has Come
    pub osvald_180: bool,

    /// Osvald 1 - Inspectors at Port
    pub osvald_190: bool,

    /// Osvald 1 - A Plan in Motion
    pub osvald_200: bool,

    /// Osvald 1 - Muzzle Removed
    pub osvald_210: bool,

    /// Osvald 1 - Davids Appears
    pub osvald_220: bool,

    /// Osvald 1 - The Answer to Escaping
    pub osvald_500: bool,

    /// Osvald 2 - No Answers Today
    pub osvald_510: bool,

    /// Osvald 2 - A Raging Fire
    pub osvald_520: bool,

    /// Osvald 2 - Escape from the Underground
    pub osvald_530: bool,

    /// Osvald 2 - Surveying the Situation
    pub osvald_540: bool,

    /// Osvald 2 - On Alert
    pub osvald_550: bool,

    /// Osvald 2 - The Answer is Straw
    pub osvald_560: bool,

    /// Osvald 2 - A Boat of Straw
    pub osvald_570: bool,

    /// Osvald 2 - Parting with Emerald
    pub osvald_580: bool,

    /// Osvald 2 - The Inspectors Leave Port
    pub osvald_590: bool,

    /// Osvald 2 - An Unsolved Mystery
    pub osvald_600: bool,

    /// Osvald 2 - Rita
    pub osvald_610: bool,

    /// Osvald 2 - Washing Ashore in Cape Cold
    pub osvald_620: bool,

    /// Osvald 2 - The Vengeful Scholar Osvald
    pub osvald_1000: bool,

    /// Osvald 3 - All I Had Was Here
    pub osvald_1010: bool,

    /// Osvald 3 - I'm Home
    pub osvald_1020: bool,

    /// Osvald 3 - Lady Clarissa
    pub osvald_1030: bool,

    /// Osvald 3 - Investigating the Incident
    pub osvald_1040: bool,

    /// Osvald 3 - Revenge Is Nothing
    pub osvald_1050: bool,

    /// Osvald 3 - Stenvar, Captain of the Guard
    pub osvald_1060: bool,

    /// Osvald 3 - Harvey's Trail
    pub osvald_1070: bool,

    /// Osvald 3 - Harvey's Trap
    pub osvald_1500: bool,

    /// Osvald 4 - Meeting Harvey
    pub osvald_1510: bool,

    /// Osvald 4 - Montwise, Town of Tomes
    pub osvald_1520: bool,

    /// Osvald 4 - Professor Harvey's Lecture
    pub osvald_1530: bool,

    /// Osvald 4 - Harvey's Whereabouts
    pub osvald_1540: bool,

    /// Osvald 4 - The Library's Hidden Passage
    pub osvald_1560: bool,

    /// Osvald 4 - Harvey's Laboratory
    pub osvald_1570: bool,

    /// Osvald 4 - The Golem
    pub osvald_1590: bool,

    /// Osvald 4 - Elena Lives
    pub osvald_1600: bool,

    /// Osvald 4 - Father and Daughter
    pub osvald_1620: bool,

    /// Osvald 4 - Osvald's Change
    pub osvald_2000: bool,

    /// Osvald 5 - Like Father, Like Daughter
    pub osvald_2010: bool,

    /// Osvald 5 - Harvey and Elena
    pub osvald_2020: bool,

    /// Osvald 5 - The Seventh Source
    pub osvald_2030: bool,

    /// Osvald 5 - Black Crystals
    pub osvald_2040: bool,

    /// Osvald 5 - Removing the Crystals
    pub osvald_2050: bool,

    /// Osvald 5 - To the Duskruin Shrine
    pub osvald_2070: bool,

    /// Osvald 5 - Experiment with the Shadow
    pub osvald_2080: bool,

    /// Osvald 5 - The One True Magic
    pub osvald_2090: bool,

    /// Osvald 5 - Elena's Research
    pub osvald_2100: bool,

    /// Osvald 5 - Osvald's Answer
    pub osvald_2110: bool,

    /// Osvald 5 - The One True Magic, Defeated
    pub osvald_2120: bool,

    /// Osvald 5 - A Daughter's Hope
    pub osvald_2130: bool,

    /// Osvald 5 - An Answer, a Journey
    pub osvald_2500: bool,

    /// Ochette Main Story Complete (End Single Story)
    pub ochette_story_complete: bool,

    /// Ochette joins the party
    pub ochette_joins: bool,

    /// Ochette 1 - A Loyal Companion
    pub ochette_10: bool,

    /// Ochette 1 - The One Not Chosen
    pub ochette_30: bool,

    /// Ochette 1 - Hunting Together
    pub ochette_40: bool,

    /// Ochette 1 - The Way of the Island
    pub ochette_50: bool,

    /// Ochette 1 - Greed Betrays You
    pub ochette_60: bool,

    /// Ochette 1 - Preparing Food
    pub ochette_70: bool,

    /// Ochette 1 - Eating Up
    pub ochette_90: bool,

    /// Ochette 1 - Cohazeh's Request
    pub ochette_100: bool,

    /// Ochette 1 - Finding the Girl
    pub ochette_130: bool,

    /// Ochette 1 - Tasty Jerky
    pub ochette_140: bool,

    /// Ochette 1 - The Islebirds' Warning
    pub ochette_150: bool,

    /// Ochette 1 - A Village in Danger
    pub ochette_160: bool,

    /// Ochette 1 - Inescapable
    pub ochette_170: bool,

    /// Ochette 1 - An Ill Omen
    pub ochette_180: bool,

    /// Ochette 1 - Three Creatures of Legend
    pub ochette_190: bool,

    /// Ochette 1 - Ochette's Mission
    pub ochette_200: bool,

    /// Ochette 1 - In Search of Legends
    pub ochette_500: bool,

    /// Ochette 2A - The Legend of Cateracta
    pub ochette_510: bool,

    /// Ochette 2A - A Strange Sound
    pub ochette_520: bool,

    /// Ochette 2A - Alpione, Guardian of the Waves
    pub ochette_530: bool,

    /// Ochette 2A - Toward the Sound
    pub ochette_540: bool,

    /// Ochette 2A - Bones
    pub ochette_550: bool,

    /// Ochette 2A - Cateracta's Last Moments
    pub ochette_560: bool,

    /// Ochette 2A - Hatching the Egg
    pub ochette_570: bool,

    /// Ochette 2A - Acta
    pub ochette_1000: bool,

    /// Ochette 2B - The Legend of Tera
    pub ochette_1010: bool,

    /// Ochette 2B - Animal Instincts
    pub ochette_1020: bool,

    /// Ochette 2B - Earthquake
    pub ochette_1030: bool,

    /// Ochette 2B - Hungry Pom
    pub ochette_1040: bool,

    /// Ochette 2B - Buttermeep Jerky
    pub ochette_1050: bool,

    /// Ochette 2B - Arrow of Awakening
    pub ochette_1060: bool,

    /// Ochette 2B - A Calmed Tera
    pub ochette_1070: bool,

    /// Ochette 2B - Pom's Goal
    pub ochette_1500: bool,

    /// Ochette 2C - The Legend of Glacis
    pub ochette_1510: bool,

    /// Ochette 2C - Glacis's Voice
    pub ochette_1520: bool,

    /// Ochette 2C - The Barrier Knight
    pub ochette_1530: bool,

    /// Ochette 2C - Sacred Peak Altahe
    pub ochette_1550: bool,

    /// Ochette 2C - A Hateful Hunter
    pub ochette_1560: bool,

    /// Ochette 2C - The Raging Glacis
    pub ochette_1570: bool,

    /// Ochette 2C - Not the Only Thing that Matters
    pub ochette_1580: bool,

    /// Ochette 2C - Descending the Mountain
    pub ochette_1590: bool,

    /// Ochette 2C - Heig's Confession
    pub ochette_1600: bool,

    /// Ochette 2C - A Blizzard Stopped
    pub ochette_2000: bool,

    /// Ochette 3 - Ochette's Return
    pub ochette_2020: bool,

    /// Ochette 3 - Preparing for Battle
    pub ochette_2030: bool,

    /// Ochette 3 - The Shadow Attacks
    pub ochette_2040: bool,

    /// Ochette 3 - A Scarlet Sky
    pub ochette_2050: bool,

    /// Ochette 3 - The Night of the Scarlet Moon
    pub ochette_2060: bool,

    /// Ochette 3 - Unfamiliar Monsters
    pub ochette_2070: bool,

    /// Ochette 3 - Beastlings and Humans Unite
    pub ochette_2080: bool,

    /// Ochette 3 - Monsters Intercepted
    pub ochette_2090: bool,

    /// Ochette 3 - Tera Attacks
    pub ochette_2100: bool,

    /// Ochette 3 - Glacis's Snow Storm
    pub ochette_2110: bool,

    /// Ochette 3 - A Familiar Scent
    pub ochette_2120: bool,

    /// Ochette 3 - On That Day
    pub ochette_2130: bool,

    /// Ochette 3 - Into the Ocean
    pub ochette_2140: bool,

    /// Ochette 3 - A Flame in the Water
    pub ochette_2150: bool,

    /// Ochette 3 - Companion's Awakening
    pub ochette_2160: bool,

    /// Ochette 3 - I'll Help You Find Peace
    pub ochette_2170: bool,

    /// Ochette 3 - If You're Reborn...
    pub ochette_2180: bool,

    /// Ochette 3 - Dawn Breaks
    pub ochette_2190: bool,

    /// Ochette 3 - Ochette, Guardian of Toto'haha
    pub ochette_2500: bool,

    /// Castti Main Story Complete (End Single Story)
    pub castti_story_complete: bool,

    /// Castti joins the party
    pub castti_joins: bool,

    /// Castti 1 - Memories
    pub castti_10: bool,

    /// Castti 1 - Lost at Sea
    pub castti_20: bool,

    /// Castti 1 - Amnesia
    pub castti_30: bool,

    /// Castti 1 - Information Gathering
    pub castti_40: bool,

    /// Castti 1 - The Satchel
    pub castti_50: bool,

    /// Castti 1 - Canalbrine Harbor
    pub castti_60: bool,

    /// Castti 1 - A Fallen Boy
    pub castti_70: bool,

    /// Castti 1 - Checkup
    pub castti_80: bool,

    /// Castti 1 - Mixing Medicine
    pub castti_90: bool,

    /// Castti 1 - Eir's Apothecaries
    pub castti_100: bool,

    /// Castti 1 - The Ill Repute of Eir's Apothecaries
    pub castti_110: bool,

    /// Castti 1 - Discovering the Source
    pub castti_120: bool,

    /// Castti 1 - Unsanitary Beasts
    pub castti_140: bool,

    /// Castti 1 - Purifying the Waters (Veron + Doron)
    pub castti_150: bool,

    /// Castti 1 - A Warm Welcome
    pub castti_160: bool,

    /// Castti 1 - Malaya the Apothecary
    pub castti_170: bool,

    /// Castti 1 - Memory Fragment: The Pier
    pub castti_180: bool,

    /// Castti 1 - Treatment Log
    pub castti_500: bool,

    /// Castti 2A - Arrival in Sai
    pub castti_510: bool,

    /// Castti 2A - Master Edmund
    pub castti_520: bool,

    /// Castti 2A - Mao the Bonemender
    pub castti_530: bool,

    /// Castti 2A - Memory Fragment: Sai's Hospital
    pub castti_540: bool,

    /// Castti 2A - The Real Eir's Apothecaries
    pub castti_550: bool,

    /// Castti 2A - Friend or Foe?
    pub castti_560: bool,

    /// Castti 2A - Helping the Injured
    pub castti_570: bool,

    /// Castti 2A - The Monster of the Sands
    pub castti_580: bool,

    /// Castti 2A - Sinking Sands
    pub castti_590: bool,

    /// Castti 2A - The Lion's Den
    pub castti_600: bool,

    /// Castti 2A - Finding the Sand Lion
    pub castti_610: bool,

    /// Castti 2A - Dealing with the Sand Lion
    pub castti_620: bool,

    /// Castti 2A - Accord
    pub castti_630: bool,

    /// Castti 2A - Rumors of Eir's Apothecaries
    pub castti_640: bool,

    /// Castti 2A - Memory Fragment: A Rainy Village
    pub castti_650: bool,

    /// Castti 2A - Memory Fragment: Rain Man
    pub castti_660: bool,

    /// Castti 2A - In Search of Fresh Clues
    pub castti_1000: bool,

    /// Castti 2B - Arrival in Winterbloom
    pub castti_1010: bool,

    /// Castti 2B - Gratitude for Eir's Apothecaries
    pub castti_1020: bool,

    /// Castti 2B - The Lady's Manor
    pub castti_1030: bool,

    /// Castti 2B - Lady Rosa
    pub castti_1040: bool,

    /// Castti 2B - The Herb Garden
    pub castti_1050: bool,

    /// Castti 2B - Melia's Wish
    pub castti_1070: bool,

    /// Castti 2B - Greg, Usurper Aspirant
    pub castti_1080: bool,

    /// Castti 2B - Even with Memories Lost
    pub castti_1090: bool,

    /// Castti 2B - Melia Kidnapped
    pub castti_1100: bool,

    /// Castti 2B - Greg's Ambitions
    pub castti_1110: bool,

    /// Castti 2B - Rogue Conflict
    pub castti_1120: bool,

    /// Castti 2B - Twelfth Birthday
    pub castti_1130: bool,

    /// Castti 2B - Memory Fragment: A Promise to Rosa
    pub castti_1140: bool,

    /// Castti 2B - Rosa's Last Breath
    pub castti_1150: bool,

    /// Castti 2B - Trousseau
    pub castti_1160: bool,

    /// Castti 2B - Memory Fragment: Trousseau
    pub castti_1170: bool,

    /// Castti 2B - In Search of Fresh Clues
    pub castti_1500: bool,

    /// Castti 3 - Ghost Town
    pub castti_1510: bool,

    /// Castti 3 - Santanejo
    pub castti_1520: bool,

    /// Castti 3 - Memory Fragment: Once Upon a Time
    pub castti_1530: bool,

    /// Castti 3 - Memory Fragment: A Sick Girl
    pub castti_1540: bool,

    /// Castti 3 - Memory Fragment: Searching for Santanejo
    pub castti_1550: bool,

    /// Castti 3 - Memory Fragment: Trousseau's Dreams
    pub castti_1560: bool,

    /// Castti 3 - What Happened Here?
    pub castti_1570: bool,

    /// Castti 3 - Memory Fragment: Disturbing Signs
    pub castti_1580: bool,

    /// Castti 3 - Memory Fragment: Purple Rain
    pub castti_1590: bool,

    /// Castti 3 - Memory Fragment: The Summit
    pub castti_1600: bool,

    /// Castti 3 - Memories of Rain
    pub castti_1610: bool,

    /// Castti 3 - Memory Fragment: The Smoke's Source
    pub castti_1620: bool,

    /// Castti 3 - Memory Fragment: Bitter Partings
    pub castti_1630: bool,

    /// Castti 3 - Memory Fragment: To the Village
    pub castti_1640: bool,

    /// Castti 3 - Memory Fragment: Escape
    pub castti_1650: bool,

    /// Castti 3 - Memory Fragment: Hope Entrusted
    pub castti_1660: bool,

    /// Castti 3 - The Truth About Malaya
    pub castti_2000: bool,

    /// Castti 4 - The Coronation Ceremony
    pub castti_2010: bool,

    /// Castti 4 - Reuniting with Edmund
    pub castti_2020: bool,

    /// Castti 4 - Griff's Support
    pub castti_2030: bool,

    /// Castti 4 - Rain, Rain
    pub castti_2040: bool,

    /// Castti 4 - Rain to Purple
    pub castti_2050: bool,

    /// Castti 4 - To Extend a Helping Hand
    pub castti_2060: bool,

    /// Castti 4 - Concocting a Cure
    pub castti_2070: bool,

    /// Castti 4 - Miracle Cure
    pub castti_2080: bool,

    /// Castti 4 - Healing Those in Need
    pub castti_2090: bool,

    /// Castti 4 - Castti Collapses
    pub castti_2100: bool,

    /// Castti 4 - Malaya's Request
    pub castti_2110: bool,

    /// Castti 4 - Partings
    pub castti_2120: bool,

    /// Castti 4 - The Deeds of Eir's Apothecaries
    pub castti_2500: bool,

    /// Hikari Main Story Complete (End Single Story)
    pub hikari_story_complete: bool,

    /// Hikari joins the party
    pub hikari_joins: bool,

    /// Hikari 1 - Before the Battle, Before Their Graves
    pub hikari_10: bool,

    /// Hikari 1 - General Mugen
    pub hikari_30: bool,

    /// Hikari 1 - Advance
    pub hikari_50: bool,

    /// Hikari 1 - Clash with the Enemy General
    pub hikari_60: bool,

    /// Hikari 1 - Ruler of the Desert
    pub hikari_70: bool,

    /// Hikari 1 - Friendly Banter
    pub hikari_80: bool,

    /// Hikari 1 - Peace Yet Prevails
    pub hikari_90: bool,

    /// Hikari 1 - Tussle at the Tavern
    pub hikari_100: bool,

    /// Hikari 1 - The Tavern Abuzz
    pub hikari_120: bool,

    /// Hikari 1 - An Unexpected Visitor
    pub hikari_130: bool,

    /// Hikari 1 - Jigo, King and Father
    pub hikari_140: bool,

    /// Hikari 1 - Merchant Found
    pub hikari_150: bool,

    /// Hikari 1 - Disquieting News
    pub hikari_170: bool,

    /// Hikari 1 - In the Throne Room
    pub hikari_190: bool,

    /// Hikari 1 - At the Crest of Heroes
    pub hikari_200: bool,

    /// Hikari 1 - A Sinister Voice
    pub hikari_210: bool,

    /// Hikari 1 - The Death of a Friend
    pub hikari_220: bool,

    /// Hikari 1 - Mugen's Treachery
    pub hikari_230: bool,

    /// Hikari 1 - Hikari's Resolve
    pub hikari_240: bool,

    /// Hikari 1 - Leaving Home Behind
    pub hikari_250: bool,

    /// Hikari 1 - In Search of Allies
    pub hikari_500: bool,

    /// Hikari 2 - The Eagle's Perch
    pub hikari_510: bool,

    /// Hikari 2 - An Unexpected Reunion
    pub hikari_520: bool,

    /// Hikari 2 - A New Challenger
    pub hikari_530: bool,

    /// Hikari 2 - Kazan's Vices
    pub hikari_540: bool,

    /// Hikari 2 - Bandelam the Reaper
    pub hikari_550: bool,

    /// Hikari 2 - Duels to the Death
    pub hikari_560: bool,

    /// Hikari 2 - The Challenge Begins
    pub hikari_580: bool,

    /// Hikari 2 - Qualifying for the Duel
    pub hikari_590: bool,

    /// Hikari 2 - Master Borneau
    pub hikari_600: bool,

    /// Hikari 2 - A Steep Wager
    pub hikari_630: bool,

    /// Hikari 2 - Zeto the Butcher
    pub hikari_640: bool,

    /// Hikari 2 - The Way of the Arena
    pub hikari_650: bool,

    /// Hikari 2 - Kazan's Lie
    pub hikari_660: bool,

    /// Hikari 2 - 300 Million Leaves
    pub hikari_670: bool,

    /// Hikari 2 - Duel with Bandelam
    pub hikari_690: bool,

    /// Hikari 2 - A Reason to Fight
    pub hikari_700: bool,

    /// Hikari 2 - The Gladiators Revolt
    pub hikari_710: bool,

    /// Hikari 2 - Gladiators Freed
    pub hikari_730: bool,

    /// Hikari 2 - New Friends
    pub hikari_1000: bool,

    /// Hikari 3 - The Sky That Day
    pub hikari_1010: bool,

    /// Hikari 3 - Meeting Ritsu
    pub hikari_1020: bool,

    /// Hikari 3 - Searching for Azuma
    pub hikari_1030: bool,

    /// Hikari 3 - The Exchange
    pub hikari_1040: bool,

    /// Hikari 3 - An Important Mission
    pub hikari_1050: bool,

    /// Hikari 3 - Weapon Transport Interrupted
    pub hikari_1060: bool,

    /// Hikari 3 - General Rou
    pub hikari_1070: bool,

    /// Hikari 3 - A Looming Shadow
    pub hikari_1080: bool,

    /// Hikari 3 - Ritsu's Ambition
    pub hikari_1090: bool,

    /// Hikari 3 - Weapons Seized
    pub hikari_1100: bool,

    /// Hikari 3 - Persuading Rai Mei
    pub hikari_1110: bool,

    /// Hikari 3 - Ritsu's Lie
    pub hikari_1500: bool,

    /// Hikari 4 - Mugen's Plan
    pub hikari_1510: bool,

    /// Hikari 4 - To Castle Mei
    pub hikari_1520: bool,

    /// Hikari 4 - Kunzo the Obstructor
    pub hikari_1530: bool,

    /// Hikari 4 - Mugen's Vassals
    pub hikari_1540: bool,

    /// Hikari 4 - Mother's Voice
    pub hikari_1550: bool,

    /// Hikari 4 - Meeting Clan Mei
    pub hikari_1560: bool,

    /// Hikari 4 - Hikari's Talents
    pub hikari_1570: bool,

    /// Hikari 4 - Clan Mei Moves Out
    pub hikari_1580: bool,

    /// Hikari 4 - A Kind Mother's Demise
    pub hikari_1590: bool,

    /// Hikari 4 - Hikari Loses Control
    pub hikari_1600: bool,

    /// Hikari 4 - A Death Sentence
    pub hikari_1610: bool,

    /// Hikari 4 - The Accursed Clan
    pub hikari_1620: bool,

    /// Hikari 4 - A Startling Truth
    pub hikari_1630: bool,

    /// Hikari 4 - Clan Leader Jin Mei
    pub hikari_1640: bool,

    /// Hikari 4 - Rai Mei's Resolve
    pub hikari_1650: bool,

    /// Hikari 4 - For Clan Mei's Survival
    pub hikari_1660: bool,

    /// Hikari 4 - Kunzo's Loyalty
    pub hikari_1670: bool,

    /// Hikari 4 - The Paths We Make
    pub hikari_1680: bool,

    /// Hikari 4 - I'll Await You in Ku
    pub hikari_2000: bool,

    /// Hikari 5 - The Prince Returns
    pub hikari_2010: bool,

    /// Hikari 5 - Ritsu and Mikka
    pub hikari_2020: bool,

    /// Hikari 5 - General Ritsu
    pub hikari_2030: bool,

    /// Hikari 5 - The Eagle's Invitation
    pub hikari_2040: bool,

    /// Hikari 5 - A Gathering of Friends
    pub hikari_2050: bool,

    /// Hikari 5 - The Sandstorm
    pub hikari_2060: bool,

    /// Hikari 5 - The Bell of Dawn
    pub hikari_2070: bool,

    /// Hikari 5 - Mugen's Order
    pub hikari_2080: bool,

    /// Hikari 5 - Ageha's Trap
    pub hikari_2090: bool,

    /// Hikari 5 - Kazan's Plan
    pub hikari_2100: bool,

    /// Hikari 5 - Rai Mei, Spear of Levin
    pub hikari_2110: bool,

    /// Hikari 5 - Diverging Paths
    pub hikari_2120: bool,

    /// Hikari 5 - The Plea of Friends
    pub hikari_2130: bool,

    /// Hikari 5 - Brothers Reunited
    pub hikari_2150: bool,

    /// Hikari 5 - Hikari's Blood
    pub hikari_2160: bool,

    /// Hikari 5 - The Darkblood Blade
    pub hikari_2170: bool,

    /// Hikari 5 - No Longer Consumed by Darkness
    pub hikari_2180: bool,

    /// Hikari 5 - A Joyous Proclamation
    pub hikari_2190: bool,

    /// Hikari 5 - Ascending the Throne
    pub hikari_2200: bool,

    /// Hikari 5 - The Blessing of Friendship
    pub hikari_2210: bool,

    /// Hikari 5 - King Hikari of Ku
    pub hikari_2220: bool,

    /// Hikari 5 - Clear Skies
    pub hikari_2500: bool,

    /// Agnea Main Story Complete (End Single Story)
    pub agnea_story_complete: bool,

    /// Agnea joins the party
    pub agnea_joins: bool,

    /// Agnea 1 - Dreams of Stardom
    pub agnea_10: bool,

    /// Agnea 1 - The Village Dancer
    pub agnea_20: bool,

    /// Agnea 1 - Seeing Off the Patrons
    pub agnea_30: bool,

    /// Agnea 1 - The Tavern After Closing
    pub agnea_40: bool,

    /// Agnea 1 - Dancing on Air
    pub agnea_50: bool,

    /// Agnea 1 - Telling Papa the News
    pub agnea_60: bool,

    /// Agnea 1 - Cuani the Star
    pub agnea_70: bool,

    /// Agnea 1 - Memories of Mama
    pub agnea_80: bool,

    /// Agnea 1 - Nothing Special
    pub agnea_90: bool,

    /// Agnea 1 - Preparing for the Festival
    pub agnea_100: bool,

    /// Agnea 1 - Pala's Gone Missing
    pub agnea_110: bool,

    /// Agnea 1 - The Raging Duorduor
    pub agnea_120: bool,

    /// Agnea 1 - A Dress Ruined
    pub agnea_130: bool,

    /// Agnea 1 - I Can Still Dance
    pub agnea_140: bool,

    /// Agnea 1 - Mama's Dress
    pub agnea_150: bool,

    /// Agnea 1 - Mama's Song
    pub agnea_160: bool,

    /// Agnea 1 - The Morning of Departure
    pub agnea_170: bool,

    /// Agnea 1 - We Will Meet Again Someday
    pub agnea_500: bool,

    /// Agnea 2 - The Metropolis
    pub agnea_510: bool,

    /// Agnea 2 - The Theater
    pub agnea_520: bool,

    /// Agnea 2 - Into the Theater
    pub agnea_550: bool,

    /// Agnea 2 - The Show Begins
    pub agnea_560: bool,

    /// Agnea 2 - Greatest Dancer in the Land
    pub agnea_570: bool,

    /// Agnea 2 - Gil the Pianist
    pub agnea_580: bool,

    /// Agnea 2 - Montraine's Tavern
    pub agnea_590: bool,

    /// Agnea 2 - A Show at the Tavern
    pub agnea_600: bool,

    /// Agnea 2 - Agnea the House Dancer
    pub agnea_610: bool,

    /// Agnea 2 - An Arrogant Man
    pub agnea_620: bool,

    /// Agnea 2 - Shattered Hopes
    pub agnea_630: bool,

    /// Agnea 2 - Finding the Manager
    pub agnea_640: bool,

    /// Agnea 2 - Manager Found
    pub agnea_650: bool,

    /// Agnea 2 - For Hope
    pub agnea_660: bool,

    /// Agnea 2 - Shining Superstar
    pub agnea_670: bool,

    /// Agnea 2 - Hope Protected
    pub agnea_680: bool,

    /// Agnea 2 - A Gift From Gil
    pub agnea_690: bool,

    /// Agnea 2 - Song of Hope
    pub agnea_700: bool,

    /// Agnea 2 - Dolcinaea's Plan
    pub agnea_1000: bool,

    /// Agnea 3 - The Traveling Troupe
    pub agnea_1010: bool,

    /// Agnea 3 - Giselle's Troupe
    pub agnea_1030: bool,

    /// Agnea 3 - The Missing Troupe Leader
    pub agnea_1040: bool,

    /// Agnea 3 - The Pressure of the Stage
    pub agnea_1050: bool,

    /// Agnea 3 - Even If You Stumble
    pub agnea_1060: bool,

    /// Agnea 3 - The Show Goes On
    pub agnea_1070: bool,

    /// Agnea 3 - A Celebratory Drink
    pub agnea_1080: bool,

    /// Agnea 3 - Promises of Reunion
    pub agnea_1500: bool,

    /// Agnea 4 - Where Mama Was
    pub agnea_1510: bool,

    /// Agnea 4 - A Mischievous Child
    pub agnea_1520: bool,

    /// Agnea 4 - Hope at Risk
    pub agnea_1530: bool,

    /// Agnea 4 - Laila Leaves Home
    pub agnea_1540: bool,

    /// Agnea 4 - Having Fun
    pub agnea_1550: bool,

    /// Agnea 4 - In Step
    pub agnea_1580: bool,

    /// Agnea 4 - The Joy of Dancing
    pub agnea_1590: bool,

    /// Agnea 4 - Cuani and Dolcinaea
    pub agnea_1600: bool,

    /// Agnea 4 - A Town in Trouble
    pub agnea_1610: bool,

    /// Agnea 4 - No Mercy
    pub agnea_1620: bool,

    /// Agnea 4 - Dolcinaealand
    pub agnea_1630: bool,

    /// Agnea 4 - A True Star
    pub agnea_1640: bool,

    /// Agnea 4 - Invitation to the Grand Gala
    pub agnea_1650: bool,

    /// Agnea 4 - Laila's Dance
    pub agnea_1660: bool,

    /// Agnea 4 - Parting from Laila
    pub agnea_2000: bool,

    /// Agnea 5 - Center of Attention
    pub agnea_2010: bool,

    /// Agnea 5 - To the Grand Gala's Stage
    pub agnea_2020: bool,

    /// Agnea 5 - The Mastermind
    pub agnea_2030: bool,

    /// Agnea 5 - Reunited with Giselle's Troupe
    pub agnea_2040: bool,

    /// Agnea 5 - Rush to the Stage
    pub agnea_2050: bool,

    /// Agnea 5 - The Grandeur of the Stage
    pub agnea_2060: bool,

    /// Agnea 5 - Gil and Laila's Show
    pub agnea_2080: bool,

    /// Agnea 5 - A Bothered Bodyguard, a Star at Ease
    pub agnea_2090: bool,

    /// Agnea 5 - The Manager's Resentment
    pub agnea_2100: bool,

    /// Agnea 5 - The Festival of Grace
    pub agnea_2110: bool,

    /// Agnea 5 - Showtime
    pub agnea_2120: bool,

    /// Agnea 5 - Hope Rekindled
    pub agnea_2130: bool,

    /// Agnea 5 - At the Dance Battle's End
    pub agnea_2140: bool,

    /// Agnea 5 - A Star without Shine
    pub agnea_2150: bool,

    /// Agnea 5 - Dolcinaea's Defeat
    pub agnea_2160: bool,

    /// Agnea 5 - Encore
    pub agnea_2170: bool,

    /// Agnea 5 - Agnea the Star
    pub agnea_2500: bool,

    // Levels
    /// Enter Abandoned Church
    pub dng_mnt_3_3_enter: bool,

    /// Exit Abandoned Church
    pub dng_mnt_3_3_exit: bool,

    /// Enter Abandoned Road
    pub fld_wld_2_3_enter: bool,

    /// Exit Abandoned Road
    pub fld_wld_2_3_exit: bool,

    /// Enter Abandoned Silver Mine
    pub fld_wld_1_1_b_enter: bool,

    /// Exit Abandoned Silver Mine
    pub fld_wld_1_1_b_exit: bool,

    /// Enter Abandoned Traverse
    pub fld_mnt_3_3_enter: bool,

    /// Exit Abandoned Traverse
    pub fld_mnt_3_3_exit: bool,

    /// Enter Abandoned Village
    pub twn_cty_1_2_a_enter: bool,

    /// Exit Abandoned Village
    pub twn_cty_1_2_a_exit: bool,

    /// Enter Abandoned Waterway
    pub dng_cty_1_3_enter: bool,

    /// Exit Abandoned Waterway
    pub dng_cty_1_3_exit: bool,

    /// Enter Abyssal Beach
    pub dng_isd_3_2_e_enter: bool,

    /// Exit Abyssal Beach
    pub dng_isd_3_2_e_exit: bool,

    /// Enter Altar of the Charitable
    pub dng_sea_2_job_enter: bool,

    /// Exit Altar of the Charitable
    pub dng_sea_2_job_exit: bool,

    /// Enter Altar of the Flamebringer
    pub dng_mnt_2_job_enter: bool,

    /// Exit Altar of the Flamebringer
    pub dng_mnt_2_job_exit: bool,

    /// Enter Altar of the Huntress
    pub dng_isd_2_job_enter: bool,

    /// Exit Altar of the Huntress
    pub dng_isd_2_job_exit: bool,

    /// Enter Altar of the Lady of Grace
    pub dng_fst_2_job_enter: bool,

    /// Exit Altar of the Lady of Grace
    pub dng_fst_2_job_exit: bool,

    /// Enter Altar of the Prince of Thieves
    pub dng_cty_2_job_enter: bool,

    /// Exit Altar of the Prince of Thieves
    pub dng_cty_2_job_exit: bool,

    /// Enter Altar of the Scholarking
    pub dng_snw_2_job_enter: bool,

    /// Exit Altar of the Scholarking
    pub dng_snw_2_job_exit: bool,

    /// Enter Altar of the Thunderblade
    pub dng_dst_2_job_enter: bool,

    /// Exit Altar of the Thunderblade
    pub dng_dst_2_job_exit: bool,

    /// Enter Altar of the Trader
    pub dng_wld_2_job_enter: bool,

    /// Exit Altar of the Trader
    pub dng_wld_2_job_exit: bool,

    /// Enter Animal Trail
    pub dng_fst_2_2_a_enter: bool,

    /// Exit Animal Trail
    pub dng_fst_2_2_a_exit: bool,

    /// Enter Battlefield
    pub fld_dst_3_1_a_enter: bool,

    /// Exit Battlefield
    pub fld_dst_3_1_a_exit: bool,

    /// Enter Beasting Bay: Anchorage
    pub fld_isd_1_3_enter: bool,

    /// Exit Beasting Bay: Anchorage
    pub fld_isd_1_3_exit: bool,

    /// Enter Beasting Village
    pub twn_isd_1_1_a_enter: bool,

    /// Exit Beasting Village
    pub twn_isd_1_1_a_exit: bool,

    /// Enter Bed of the Titan
    pub dng_wld_2_1_b_enter: bool,

    /// Exit Bed of the Titan
    pub dng_wld_2_1_b_exit: bool,

    /// Enter Beneath the Wall
    pub dng_snw_3_4_a_enter: bool,

    /// Exit Beneath the Wall
    pub dng_snw_3_4_a_exit: bool,

    /// Enter Borderfall
    pub fld_mnt_2_2_enter: bool,

    /// Exit Borderfall
    pub fld_mnt_2_2_exit: bool,

    /// Enter Canalbrine
    pub twn_sea_1_1_a_enter: bool,

    /// Exit Canalbrine
    pub twn_sea_1_1_a_exit: bool,

    /// Enter Canalbrine Bridge
    pub fld_sea_1_2_enter: bool,

    /// Exit Canalbrine Bridge
    pub fld_sea_1_2_exit: bool,

    /// Enter Canalbrine: Path to the Water Source
    pub fld_sea_1_1_enter: bool,

    /// Exit Canalbrine: Path to the Water Source
    pub fld_sea_1_1_exit: bool,

    /// Enter Canalbrine: Water Source
    pub dng_sea_1_1_enter: bool,

    /// Exit Canalbrine: Water Source
    pub dng_sea_1_1_exit: bool,

    /// Enter Cape Cold
    pub twn_snw_1_2_a_enter: bool,

    /// Exit Cape Cold
    pub twn_snw_1_2_a_exit: bool,

    /// Enter Castle Ku
    pub twn_dst_3_1_c_enter: bool,

    /// Exit Castle Ku
    pub twn_dst_3_1_c_exit: bool,

    /// Enter Castle Ku: Entrance
    pub twn_dst_3_1_b_enter: bool,

    /// Exit Castle Ku: Entrance
    pub twn_dst_3_1_b_exit: bool,

    /// Enter Castle Mei: East Tower
    pub dng_snw_3_2_a_enter: bool,

    /// Exit Castle Mei: East Tower
    pub dng_snw_3_2_a_exit: bool,

    /// Enter Castle Mei: Gallows
    pub dng_snw_3_2_b_enter: bool,

    /// Exit Castle Mei: Gallows
    pub dng_snw_3_2_b_exit: bool,

    /// Enter Castle Vidania
    pub dng_atl_3_1_enter: bool,

    /// Exit Castle Vidania
    pub dng_atl_3_1_exit: bool,

    /// Enter Cathedral Cellars
    pub dng_mnt_1_1_enter: bool,

    /// Exit Cathedral Cellars
    pub dng_mnt_1_1_exit: bool,

    /// Enter Cavern of the Moon and Sun
    pub dng_sea_2_3_enter: bool,

    /// Exit Cavern of the Moon and Sun
    pub dng_sea_2_3_exit: bool,

    /// Enter Cavern of the Sea God
    pub dng_sea_2_1_enter: bool,

    /// Exit Cavern of the Sea God
    pub dng_sea_2_1_exit: bool,

    /// Enter Cavern of Waves
    pub dng_isd_1_2_enter: bool,

    /// Exit Cavern of Waves
    pub dng_isd_1_2_exit: bool,

    /// Enter Clockbank
    pub twn_cty_2_1_a_enter: bool,

    /// Exit Clockbank
    pub twn_cty_2_1_a_exit: bool,

    /// Enter Clockbank: Industrial District
    pub twn_cty_2_1_b_enter: bool,

    /// Exit Clockbank: Industrial District
    pub twn_cty_2_1_b_exit: bool,

    /// Enter Conning Creek
    pub twn_sea_2_1_a_enter: bool,

    /// Exit Conning Creek
    pub twn_sea_2_1_a_exit: bool,

    /// Enter Conning Creek: Harbor
    pub twn_sea_2_1_b_enter: bool,

    /// Exit Conning Creek: Harbor
    pub twn_sea_2_1_b_exit: bool,

    /// Enter Conning Creek: Outskirts
    pub twn_sea_2_1_c_enter: bool,

    /// Exit Conning Creek: Outskirts
    pub twn_sea_2_1_c_exit: bool,

    /// Enter Crackridge
    pub twn_wld_2_1_a_enter: bool,

    /// Exit Crackridge
    pub twn_wld_2_1_a_exit: bool,

    /// Enter Crackridge Harbor: Anchorage
    pub fld_wld_1_2_enter: bool,

    /// Exit Crackridge Harbor: Anchorage
    pub fld_wld_1_2_exit: bool,

    /// Enter Cropdale
    pub twn_fst_1_1_a_enter: bool,

    /// Exit Cropdale
    pub twn_fst_1_1_a_exit: bool,

    /// Enter Curious Nest
    pub dng_ocn_1_2_enter: bool,

    /// Exit Curious Nest
    pub dng_ocn_1_2_exit: bool,

    /// Enter Dark Night
    pub dng_fst_2_2_b_enter: bool,

    /// Exit Dark Night
    pub dng_fst_2_2_b_exit: bool,

    /// Enter Decaying Temple
    pub dng_dst_2_3_enter: bool,

    /// Exit Decaying Temple
    pub dng_dst_2_3_exit: bool,

    /// Enter Deserted Highroad
    pub fld_cty_3_1_enter: bool,

    /// Exit Deserted Highroad
    pub fld_cty_3_1_exit: bool,

    /// Enter Diamante's Estate
    pub dng_cty_1_1_enter: bool,

    /// Exit Diamante's Estate
    pub dng_cty_1_1_exit: bool,

    /// Enter Dragonridge
    pub dng_dst_2_1_enter: bool,

    /// Exit Dragonridge
    pub dng_dst_2_1_exit: bool,

    /// Enter Duskruin Shrine
    pub dng_wld_3_1_a_enter: bool,

    /// Exit Duskruin Shrine
    pub dng_wld_3_1_a_exit: bool,

    /// Enter Duskruin Shrine: Depths
    pub dng_wld_3_1_b_enter: bool,

    /// Exit Duskruin Shrine: Depths
    pub dng_wld_3_1_b_exit: bool,

    /// Enter Eastern Cape Cold Snows
    pub fld_snw_1_3_enter: bool,

    /// Exit Eastern Cape Cold Snows
    pub fld_snw_1_3_exit: bool,

    /// Enter Eastern Cropdale Trail
    pub fld_fst_1_3_enter: bool,

    /// Exit Eastern Cropdale Trail
    pub fld_fst_1_3_exit: bool,

    /// Enter Eastern Flamechurch Pass
    pub fld_mnt_1_2_enter: bool,

    /// Exit Eastern Flamechurch Pass
    pub fld_mnt_1_2_exit: bool,

    /// Enter Eastern Ku Sands
    pub fld_dst_3_2_enter: bool,

    /// Exit Eastern Ku Sands
    pub fld_dst_3_2_exit: bool,

    /// Enter Eastern New Delsta Highroad
    pub fld_cty_1_3_enter: bool,

    /// Exit Eastern New Delsta Highroad
    pub fld_cty_1_3_exit: bool,

    /// Enter Eastern Sai Sands
    pub fld_dst_2_4_enter: bool,

    /// Exit Eastern Sai Sands
    pub fld_dst_2_4_exit: bool,

    /// Enter Eastern Wellgrove Trail
    pub fld_fst_2_2_enter: bool,

    /// Exit Eastern Wellgrove Trail
    pub fld_fst_2_2_exit: bool,

    /// Enter Fellsun Ruins
    pub dng_wld_2_2_enter: bool,

    /// Exit Fellsun Ruins
    pub dng_wld_2_2_exit: bool,

    /// Enter Festival Grounds
    pub fld_fst_1_2_enter: bool,

    /// Exit Festival Grounds
    pub fld_fst_1_2_exit: bool,

    /// Enter Five-Tiered Tower
    pub dng_dst_3_2_a_enter: bool,

    /// Exit Five-Tiered Tower
    pub dng_dst_3_2_a_exit: bool,

    /// Enter Five-Tiered Tower: Fourth Floor
    pub dng_dst_3_2_d_enter: bool,

    /// Exit Five-Tiered Tower: Fourth Floor
    pub dng_dst_3_2_d_exit: bool,

    /// Enter Five-Tiered Tower: Second Floor
    pub dng_dst_3_2_b_enter: bool,

    /// Exit Five-Tiered Tower: Second Floor
    pub dng_dst_3_2_b_exit: bool,

    /// Enter Five-Tiered Tower: Third Floor
    pub dng_dst_3_2_c_enter: bool,

    /// Exit Five-Tiered Tower: Third Floor
    pub dng_dst_3_2_c_exit: bool,

    /// Enter Five-Tiered Tower: Top Floor
    pub dng_dst_3_2_e_enter: bool,

    /// Exit Five-Tiered Tower: Top Floor
    pub dng_dst_3_2_e_exit: bool,

    /// Enter Flamechurch
    pub twn_mnt_1_1_a_enter: bool,

    /// Exit Flamechurch
    pub twn_mnt_1_1_a_exit: bool,

    /// Enter Flamechurch Pilgrims' Way
    pub fld_mnt_1_1_enter: bool,

    /// Exit Flamechurch Pilgrims' Way
    pub fld_mnt_1_1_exit: bool,

    /// Enter Flamechurch: Cathedral
    pub twn_mnt_1_2_b_enter: bool,

    /// Exit Flamechurch: Cathedral
    pub twn_mnt_1_2_b_exit: bool,

    /// Enter Flamechurch: Cathedral Entrance
    pub twn_mnt_1_2_a_enter: bool,

    /// Exit Flamechurch: Cathedral Entrance
    pub twn_mnt_1_2_a_exit: bool,

    /// Enter Forbidden Shrine
    pub dng_snw_3_1_enter: bool,

    /// Exit Forbidden Shrine
    pub dng_snw_3_1_exit: bool,

    /// Enter Forest Path
    pub fld_fst_1_1_enter: bool,

    /// Exit Forest Path
    pub fld_fst_1_1_exit: bool,

    /// Enter Forsaken Graveyard
    pub dng_mnt_2_1_enter: bool,

    /// Exit Forsaken Graveyard
    pub dng_mnt_2_1_exit: bool,

    /// Enter Frigit Isle: Anchorage
    pub fld_snw_1_1_b_enter: bool,

    /// Exit Frigit Isle: Anchorage
    pub fld_snw_1_1_b_exit: bool,

    /// Enter Frigit Isle: Entrance
    pub fld_snw_1_1_a_enter: bool,

    /// Exit Frigit Isle: Entrance
    pub fld_snw_1_1_a_exit: bool,

    /// Enter Frigit Isle: Mining Site
    pub twn_snw_1_1_c_enter: bool,

    /// Exit Frigit Isle: Mining Site
    pub twn_snw_1_1_c_exit: bool,

    /// Enter Frigit Isle: Prison
    pub twn_snw_1_1_a_enter: bool,

    /// Exit Frigit Isle: Prison
    pub twn_snw_1_1_a_exit: bool,

    /// Enter Frigit Isle: Yard
    pub twn_snw_1_1_b_enter: bool,

    /// Exit Frigit Isle: Yard
    pub twn_snw_1_1_b_exit: bool,

    /// Enter Gate of ㌀㌁㌂㌃㌄
    pub dng_ocn_1_4_enter: bool,

    /// Exit Gate of ㌀㌁㌂㌃㌄
    pub dng_ocn_1_4_exit: bool,

    /// Enter Giff's Manse
    pub dng_wld_1_1_enter: bool,

    /// Exit Giff's Manse
    pub dng_wld_1_1_exit: bool,

    /// Enter Gravell
    pub twn_wld_3_1_a_enter: bool,

    /// Exit Gravell
    pub twn_wld_3_1_a_exit: bool,

    /// Enter Guard Outpost
    pub dng_sea_2_2_enter: bool,

    /// Exit Guard Outpost
    pub dng_sea_2_2_exit: bool,

    /// Enter Healeaks
    pub twn_cty_1_2_b_enter: bool,

    /// Exit Healeaks
    pub twn_cty_1_2_b_exit: bool,

    /// Enter House Wellows Manor
    pub dng_fst_3_3_enter: bool,

    /// Exit House Wellows Manor
    pub dng_fst_3_3_exit: bool,

    /// Enter Infernal Castle
    pub dng_snw_3_4_b_enter: bool,

    /// Exit Infernal Castle
    pub dng_snw_3_4_b_exit: bool,

    /// Enter Ivory Ravine
    pub dng_wld_3_2_enter: bool,

    /// Exit Ivory Ravine
    pub dng_wld_3_2_exit: bool,

    /// Enter Ku: Castle Town
    pub twn_dst_3_1_a_enter: bool,

    /// Enter Ku: Castle Town
    pub twn_dst_3_1_a_fire_enter: bool,

    /// Exit Ku: Castle Town
    pub twn_dst_3_1_a_exit: bool,

    /// Exit Ku: Castle Town
    pub twn_dst_3_1_a_fire_exit: bool,

    /// Enter Lair of the Usurper
    pub dng_sea_1_3_enter: bool,

    /// Exit Lair of the Usurper
    pub dng_sea_1_3_exit: bool,

    /// Enter Lighthouse Island
    pub fld_ocn_1_2_enter: bool,

    /// Exit Lighthouse Island
    pub fld_ocn_1_2_exit: bool,

    /// Enter Lostseed
    pub twn_cty_3_1_a_enter: bool,

    /// Exit Lostseed
    pub twn_cty_3_1_a_exit: bool,

    /// Enter Lostseed Castle
    pub dng_cty_3_2_a_enter: bool,

    /// Exit Lostseed Castle
    pub dng_cty_3_2_a_exit: bool,

    /// Enter Lostseed Castle: Upper Level
    pub dng_cty_3_2_b_enter: bool,

    /// Exit Lostseed Castle: Upper Level
    pub dng_cty_3_2_b_exit: bool,

    /// Enter Merry Hills
    pub twn_mnt_3_1_a_enter: bool,

    /// Exit Merry Hills
    pub twn_mnt_3_1_a_exit: bool,

    /// Enter Merry Hills: Shrine Entrance
    pub twn_mnt_3_1_b_enter: bool,

    /// Exit Merry Hills: Shrine Entrance
    pub twn_mnt_3_1_b_exit: bool,

    /// Enter Montwise
    pub twn_mnt_2_1_a_enter: bool,

    /// Exit Montwise
    pub twn_mnt_2_1_a_exit: bool,

    /// Enter Montwise: Library
    pub twn_mnt_2_1_b_enter: bool,

    /// Exit Montwise: Library
    pub twn_mnt_2_1_b_exit: bool,

    /// Enter Montwise: Underground Arena
    pub twn_mnt_2_1_c_enter: bool,

    /// Exit Montwise: Underground Arena
    pub twn_mnt_2_1_c_exit: bool,

    /// Enter Moonview Cliff
    pub dng_isd_3_2_d_enter: bool,

    /// Exit Moonview Cliff
    pub dng_isd_3_2_d_exit: bool,

    /// Enter Mother's Garden
    pub dng_fst_3_2_enter: bool,

    /// Exit Mother's Garden
    pub dng_fst_3_2_exit: bool,

    /// Enter Mother's Garden: Orphanage
    pub twn_fst_3_2_a_enter: bool,

    /// Exit Mother's Garden: Orphanage
    pub twn_fst_3_2_a_exit: bool,

    /// Enter Mount Liphia
    pub dng_cty_2_2_enter: bool,

    /// Exit Mount Liphia
    pub dng_cty_2_2_exit: bool,

    /// Enter Nameless Isle
    pub dng_ocn_1_3_enter: bool,

    /// Exit Nameless Isle
    pub dng_ocn_1_3_exit: bool,

    /// Enter Nameless Village
    pub twn_isd_3_1_a_enter: bool,

    /// Exit Nameless Village
    pub twn_isd_3_1_a_exit: bool,

    /// Enter New Delsta
    pub twn_cty_1_1_a_enter: bool,

    /// Exit New Delsta
    pub twn_cty_1_1_a_exit: bool,

    /// Enter New Delsta Flats
    pub fld_cty_1_2_enter: bool,

    /// Exit New Delsta Flats
    pub fld_cty_1_2_exit: bool,

    /// Enter New Delsta Harbor: Anchorage
    pub fld_cty_1_1_enter: bool,

    /// Exit New Delsta Harbor: Anchorage
    pub fld_cty_1_1_exit: bool,

    /// Enter New Delsta: Backstreets
    pub twn_cty_1_1_b_enter: bool,

    /// Exit New Delsta: Backstreets
    pub twn_cty_1_1_b_exit: bool,

    /// Enter New Delsta: Game Parlor
    pub twn_cty_1_1_c_enter: bool,

    /// Exit New Delsta: Game Parlor
    pub twn_cty_1_1_c_exit: bool,

    /// Enter North Beasting Traverse
    pub fld_isd_1_2_enter: bool,

    /// Exit North Beasting Traverse
    pub fld_isd_1_2_exit: bool,

    /// Enter Northern Conning Creek Coast
    pub fld_sea_2_2_enter: bool,

    /// Exit Northern Conning Creek Coast
    pub fld_sea_2_2_exit: bool,

    /// Enter Northern Montwise Pass
    pub fld_mnt_3_1_enter: bool,

    /// Exit Northern Montwise Pass
    pub fld_mnt_3_1_exit: bool,

    /// Enter Northern Ryu Sands
    pub fld_dst_1_1_enter: bool,

    /// Exit Northern Ryu Sands
    pub fld_dst_1_1_exit: bool,

    /// Enter Northern Wellgrove Trail
    pub fld_fst_2_1_enter: bool,

    /// Exit Northern Wellgrove Trail
    pub fld_fst_2_1_exit: bool,

    /// Enter Ocean ???
    pub fld_ocn_1_4_enter: bool,

    /// Exit Ocean ???
    pub fld_ocn_1_4_exit: bool,

    /// Enter Old Campsite
    pub fld_dst_2_5_b_enter: bool,

    /// Exit Old Campsite
    pub fld_dst_2_5_b_exit: bool,

    /// Enter Old Clock Tower
    pub dng_cty_2_3_enter: bool,

    /// Exit Old Clock Tower
    pub dng_cty_2_3_exit: bool,

    /// Enter On the Water
    pub fld_ocn_1_1_enter: bool,

    /// Exit On the Water
    pub fld_ocn_1_1_exit: bool,

    /// Enter Oresrush A
    pub twn_wld_1_1_a_enter: bool,

    /// Enter Oresrush B
    pub twn_wld_1_1_b_enter: bool,

    /// Enter Oresrush C
    pub twn_wld_1_1_c_enter: bool,

    /// Exit Oresrush A
    pub twn_wld_1_1_a_exit: bool,

    /// Exit Oresrush B
    pub twn_wld_1_1_b_exit: bool,

    /// Exit Oresrush C
    pub twn_wld_1_1_c_exit: bool,

    /// Enter Oresrush: Foundry
    pub twn_wld_1_2_b_enter: bool,

    /// Enter Oresrush: Foundry
    pub twn_wld_1_2_a_enter: bool,

    /// Exit Oresrush: Foundry
    pub twn_wld_1_2_b_exit: bool,

    /// Exit Oresrush: Foundry
    pub twn_wld_1_2_a_exit: bool,

    /// Enter Path to Mount Liphia
    pub fld_cty_2_2_enter: bool,

    /// Exit Path to Mount Liphia
    pub fld_cty_2_2_exit: bool,

    /// Enter Path to the Bed of the Titan
    pub dng_wld_2_1_a_enter: bool,

    /// Exit Path to the Bed of the Titan
    pub dng_wld_2_1_a_exit: bool,

    /// Enter Path to the Duskruin Shrine
    pub fld_wld_3_1_enter: bool,

    /// Exit Path to the Duskruin Shrine
    pub fld_wld_3_1_exit: bool,

    /// Enter Path to the Tombs of the Wardenbeasts
    pub fld_isd_1_1_enter: bool,

    /// Exit Path to the Tombs of the Wardenbeasts
    pub fld_isd_1_1_exit: bool,

    /// Enter Prison: Underground Passage
    pub dng_snw_1_1_enter: bool,

    /// Exit Prison: Underground Passage
    pub dng_snw_1_1_exit: bool,

    /// Enter Quicksand Gaol
    pub dng_dst_2_2_enter: bool,

    /// Exit Quicksand Gaol
    pub dng_dst_2_2_exit: bool,

    /// Enter Rifted Rock
    pub dng_isd_3_1_c_enter: bool,

    /// Exit Rifted Rock
    pub dng_isd_3_1_c_exit: bool,

    /// Enter Road to Mother's Garden
    pub fld_fst_3_1_enter: bool,

    /// Exit Road to Mother's Garden
    pub fld_fst_3_1_exit: bool,

    /// Enter Roque Island
    pub twn_sea_3_1_a_enter: bool,

    /// Exit Roque Island
    pub twn_sea_3_1_a_exit: bool,

    /// Enter Roque Island: Anchorage
    pub fld_sea_3_1_enter: bool,

    /// Exit Roque Island: Anchorage
    pub fld_sea_3_1_exit: bool,

    /// Enter Roque Island: Headquarters
    pub twn_sea_3_1_b_enter: bool,

    /// Exit Roque Island: Headquarters
    pub twn_sea_3_1_b_exit: bool,

    /// Enter Ruffians' Hideout
    pub dng_snw_1_2_enter: bool,

    /// Exit Ruffians' Hideout
    pub dng_snw_1_2_exit: bool,

    /// Enter Ryu
    pub twn_dst_1_1_a_enter: bool,

    /// Exit Ryu
    pub twn_dst_1_1_a_exit: bool,

    /// Enter Sacred Guard Ship
    pub dng_sea_1_2_enter: bool,

    /// Exit Sacred Guard Ship
    pub dng_sea_1_2_exit: bool,

    /// Enter Sacred Peak Altahe
    pub dng_snw_3_3_enter: bool,

    /// Exit Sacred Peak Altahe
    pub dng_snw_3_3_exit: bool,

    /// Enter Sai
    pub twn_dst_2_1_a_enter: bool,

    /// Exit Sai
    pub twn_dst_2_1_a_exit: bool,

    /// Enter Sai: East District
    pub twn_dst_2_1_b_enter: bool,

    /// Exit Sai: East District
    pub twn_dst_2_1_b_exit: bool,

    /// Enter Sand Lion's Den
    pub dng_dst_2_4_enter: bool,

    /// Exit Sand Lion's Den
    pub dng_dst_2_4_exit: bool,

    /// Enter Sandflow Pass
    pub fld_dst_2_5_a_enter: bool,

    /// Exit Sandflow Pass
    pub fld_dst_2_5_a_exit: bool,

    /// Enter Seat of the Water Sprite
    pub dng_mnt_2_2_enter: bool,

    /// Exit Seat of the Water Sprite
    pub dng_mnt_2_2_exit: bool,

    /// Enter Secret Forest
    pub dng_fst_2_1_enter: bool,

    /// Exit Secret Forest
    pub dng_fst_2_1_exit: bool,

    /// Enter Shipwreck of the Empress
    pub dng_ocn_1_1_enter: bool,

    /// Exit Shipwreck of the Empress
    pub dng_ocn_1_1_exit: bool,

    /// Enter Shrine of Ul'sterra
    pub dng_mnt_3_1_a_enter: bool,

    /// Exit Shrine of Ul'sterra
    pub dng_mnt_3_1_a_exit: bool,

    /// Enter Silver Mine
    pub fld_wld_1_1_a_enter: bool,

    /// Exit Silver Mine
    pub fld_wld_1_1_a_exit: bool,

    /// Enter Sinking Ruins
    pub dng_isd_2_1_enter: bool,

    /// Exit Sinking Ruins
    pub dng_isd_2_1_exit: bool,

    /// Enter Snowhares' Den
    pub dng_snw_2_1_enter: bool,

    /// Exit Snowhares' Den
    pub dng_snw_2_1_exit: bool,

    /// Enter Southern Cape Cold Snows
    pub fld_snw_1_2_enter: bool,

    /// Exit Southern Cape Cold Snows
    pub fld_snw_1_2_exit: bool,

    /// Enter Southern Clockbank Highroad
    pub fld_cty_2_1_enter: bool,

    /// Exit Southern Clockbank Highroad
    pub fld_cty_2_1_exit: bool,

    /// Enter Southern Crackridge Wilds
    pub fld_wld_2_2_enter: bool,

    /// Exit Southern Crackridge Wilds
    pub fld_wld_2_2_exit: bool,

    /// Enter Southern Cropdale Trail
    pub fld_fst_1_4_enter: bool,

    /// Exit Southern Cropdale Trail
    pub fld_fst_1_4_exit: bool,

    /// Enter Southern Ku Sands
    pub fld_dst_3_1_b_enter: bool,

    /// Exit Southern Ku Sands
    pub fld_dst_3_1_b_exit: bool,

    /// Enter Southern Nameless Village Traverse
    pub fld_isd_3_3_enter: bool,

    /// Exit Southern Nameless Village Traverse
    pub fld_isd_3_3_exit: bool,

    /// Enter Southern Oresrush Wilds
    pub fld_wld_1_3_enter: bool,

    /// Exit Southern Oresrush Wilds
    pub fld_wld_1_3_exit: bool,

    /// Enter Southern Sai Sands
    pub fld_dst_2_3_enter: bool,

    /// Exit Southern Sai Sands
    pub fld_dst_2_3_exit: bool,

    /// Enter Southern Stormhail Snows
    pub fld_snw_3_1_enter: bool,

    /// Exit Southern Stormhail Snows
    pub fld_snw_3_1_exit: bool,

    /// Enter Southern Timberain Trail
    pub fld_fst_3_2_enter: bool,

    /// Exit Southern Timberain Trail
    pub fld_fst_3_2_exit: bool,

    /// Enter Stage of the Moon and Sun
    pub dng_mnt_3_1_b_enter: bool,

    /// Exit Stage of the Moon and Sun
    pub dng_mnt_3_1_b_exit: bool,

    /// Enter Starfall Spring
    pub dng_fst_1_2_enter: bool,

    /// Exit Starfall Spring
    pub dng_fst_1_2_exit: bool,

    /// Enter Stormhail
    pub twn_snw_3_1_a_enter: bool,

    /// Exit Stormhail
    pub twn_snw_3_1_a_exit: bool,

    /// Enter Stormhail: Bridge
    pub twn_snw_3_1_c_enter: bool,

    /// Exit Stormhail: Bridge
    pub twn_snw_3_1_c_exit: bool,

    /// Enter Stormhail: Castle Mei
    pub twn_snw_3_2_a_enter: bool,

    /// Exit Stormhail: Castle Mei
    pub twn_snw_3_2_a_exit: bool,

    /// Enter Stormhail: Sacred Guard Headquarters
    pub twn_snw_3_1_b_enter: bool,

    /// Exit Stormhail: Sacred Guard Headquarters
    pub twn_snw_3_1_b_exit: bool,

    /// Enter Stormy Cape
    pub dng_isd_3_2_c_enter: bool,

    /// Exit Stormy Cape
    pub dng_isd_3_2_c_exit: bool,

    /// Enter Summit of Strife
    pub dng_isd_3_2_a_enter: bool,

    /// Exit Summit of Strife
    pub dng_isd_3_2_a_exit: bool,

    /// Enter Sunken Maw
    pub dng_cty_1_4_enter: bool,

    /// Exit Sunken Maw
    pub dng_cty_1_4_exit: bool,

    /// Enter The Lost Isle
    pub fld_ocn_1_3_enter: bool,

    /// Exit The Lost Isle
    pub fld_ocn_1_3_exit: bool,

    /// Enter The Roque Company: East Tower
    pub dng_sea_3_1_b_enter: bool,

    /// Exit The Roque Company: East Tower
    pub dng_sea_3_1_b_exit: bool,

    /// Enter The Roque Company: Factory
    pub dng_cty_2_1_enter: bool,

    /// Exit The Roque Company: Factory
    pub dng_cty_2_1_exit: bool,

    /// Enter The Roque Company: West Tower
    pub dng_sea_3_1_a_enter: bool,

    /// Exit The Roque Company: West Tower
    pub dng_sea_3_1_a_exit: bool,

    /// Enter Theater
    pub dng_cty_1_2_enter: bool,

    /// Exit Theater
    pub dng_cty_1_2_exit: bool,

    /// Enter Timberain
    pub twn_fst_3_1_a_enter: bool,

    /// Exit Timberain
    pub twn_fst_3_1_a_exit: bool,

    /// Enter Timberain Castle
    pub dng_fst_3_1_a_enter: bool,

    /// Exit Timberain Castle
    pub dng_fst_3_1_a_exit: bool,

    /// Enter Timberain Castle: Roof
    pub dng_fst_3_1_b_enter: bool,

    /// Exit Timberain Castle: Roof
    pub dng_fst_3_1_b_exit: bool,

    /// Enter Timberain Castle: Town Square
    pub twn_fst_3_1_b_enter: bool,

    /// Exit Timberain Castle: Town Square
    pub twn_fst_3_1_b_exit: bool,

    /// Enter Tombs of the Wardenbeasts
    pub dng_isd_1_1_enter: bool,

    /// Exit Tombs of the Wardenbeasts
    pub dng_isd_1_1_exit: bool,

    /// Enter Tranquil Grotto
    pub dng_dst_3_1_enter: bool,

    /// Exit Tranquil Grotto
    pub dng_dst_3_1_exit: bool,

    /// Enter Tropu'hopu
    pub twn_isd_2_1_a_enter: bool,

    /// Exit Tropu'hopu
    pub twn_isd_2_1_a_exit: bool,

    /// Enter Tropu'hopu: Floating Theater
    pub twn_isd_2_1_b_enter: bool,

    /// Exit Tropu'hopu: Floating Theater
    pub twn_isd_2_1_b_exit: bool,

    /// Enter Tropu'hopu: Shipyard
    pub twn_isd_2_1_c_enter: bool,

    /// Exit Tropu'hopu: Shipyard
    pub twn_isd_2_1_c_exit: bool,

    /// Enter Underground Laboratory
    pub dng_mnt_2_3_enter: bool,

    /// Exit Underground Laboratory
    pub dng_mnt_2_3_exit: bool,

    /// Enter Underground Waterway
    pub dng_cty_3_1_enter: bool,

    /// Exit Underground Waterway
    pub dng_cty_3_1_exit: bool,

    /// Enter Unfinished Tunnel
    pub dng_wld_1_2_enter: bool,

    /// Exit Unfinished Tunnel
    pub dng_wld_1_2_exit: bool,

    /// Enter unknown
    pub evt_end_30_001_enter: bool,

    /// Enter Veil of Trees
    pub dng_fst_1_1_enter: bool,

    /// Exit Veil of Trees
    pub dng_fst_1_1_exit: bool,

    /// Enter Verdant Wood
    pub dng_isd_3_2_b_enter: bool,

    /// Exit Verdant Wood
    pub dng_isd_3_2_b_exit: bool,

    /// Enter Vidania
    pub fld_atl_3_1_enter: bool,

    /// Exit Vidania
    pub fld_atl_3_1_exit: bool,

    /// Enter Wandering Wood A
    pub dng_isd_3_1_a_enter: bool,

    /// Enter Wandering Wood B
    pub dng_isd_3_1_b_enter: bool,

    /// Exit Wandering Wood A
    pub dng_isd_3_1_a_exit: bool,

    /// Exit Wandering Wood B
    pub dng_isd_3_1_b_exit: bool,

    /// Enter Wellgrove
    pub twn_fst_2_1_a_enter: bool,

    /// Exit Wellgrove
    pub twn_fst_2_1_a_exit: bool,

    /// Enter Wellgrove: Alrond's Estate
    pub twn_fst_2_1_b_enter: bool,

    /// Exit Wellgrove: Alrond's Estate
    pub twn_fst_2_1_b_exit: bool,

    /// Enter Western Canalbrine Coast
    pub fld_sea_1_3_enter: bool,

    /// Exit Western Canalbrine Coast
    pub fld_sea_1_3_exit: bool,

    /// Enter Western Clockbank Highroad
    pub fld_cty_1_4_enter: bool,

    /// Exit Western Clockbank Highroad
    pub fld_cty_1_4_exit: bool,

    /// Enter Western Conning Creek Coast
    pub fld_sea_2_3_enter: bool,

    /// Exit Western Conning Creek Coast
    pub fld_sea_2_3_exit: bool,

    /// Enter Western Crackridge Wilds
    pub fld_wld_2_1_enter: bool,

    /// Exit Western Crackridge Wilds
    pub fld_wld_2_1_exit: bool,

    /// Enter Western Gravell Wilds
    pub fld_wld_3_2_enter: bool,

    /// Exit Western Gravell Wilds
    pub fld_wld_3_2_exit: bool,

    /// Enter Western Merry Hills Pass
    pub fld_mnt_3_2_enter: bool,

    /// Exit Western Merry Hills Pass
    pub fld_mnt_3_2_exit: bool,

    /// Enter Western Montwise Pass
    pub fld_mnt_2_1_enter: bool,

    /// Exit Western Montwise Pass
    pub fld_mnt_2_1_exit: bool,

    /// Enter Western Sai Sands
    pub fld_dst_2_2_enter: bool,

    /// Exit Western Sai Sands
    pub fld_dst_2_2_exit: bool,

    /// Enter Western Tropu'hopu Traverse
    pub fld_isd_2_1_enter: bool,

    /// Exit Western Tropu'hopu Traverse
    pub fld_isd_2_1_exit: bool,

    /// Enter Western Winterbloom Snows
    pub fld_snw_2_2_enter: bool,

    /// Exit Western Winterbloom Snows
    pub fld_snw_2_2_exit: bool,

    /// Enter Winterbloom
    pub twn_snw_2_1_a_enter: bool,

    /// Exit Winterbloom
    pub twn_snw_2_1_a_exit: bool,

    /// Enter Winterbloom: Thieves' Quarter
    pub twn_snw_2_1_b_enter: bool,

    /// Exit Winterbloom: Thieves' Quarter
    pub twn_snw_2_1_b_exit: bool,
}
