use crate::board::{Board, Color::Black, make_move::Action, types::{BLACK_BISHOP, BLACK_KING, BLACK_KNIGHT, BLACK_PAWN, BLACK_QUEEN, BLACK_ROOK, Move, NO_SQUARE, WHITE_BISHOP, WHITE_KING, WHITE_KNIGHT, WHITE_PAWN, WHITE_QUEEN, WHITE_ROOK}};

pub const PIECE_SQUARE_KEYS: [[u64; 64]; 12] = [
  [0xe4697ccaa7322bdf, 0x5a9ebf660b1d5848, 0xb99bcc514aa470f8, 0x4c0e58abc3000619, 
   0xab01be0314eea055, 0x23b7a5ec6dcfc855, 0xaeae43dcfae6e285, 0x886a3acf7afb2e00, 
   0xaadbcc318fbd585c, 0x5e8017c2c327b5ec, 0x22b24cdb7aff0187, 0x887856376f727e44, 
   0x0ee6e359858b8bb8, 0x43b68265c04b72af, 0x0eed209d2856af8a, 0x5a92dc2592f6a055, 
   0x845b224440dbf855, 0xfbc48e567464fae5, 0xd26ce1870a4e2a50, 0x3f5e1dfb16922204, 
   0x86b6bd390ef70740, 0x2e0ffc58535f394e, 0x2f9859e3fa6614bc, 0xf929a3e289aa7795, 
   0x7eab889ea4a2663a, 0x56c2d7d92559ec76, 0x0d087959f8ba822e, 0x03a36b9f4cfabcaa, 
   0x280515b65dff2353, 0xf8cdec79794c69d5, 0xc864edea1d122446, 0x6f5b8f476a35d18e, 
   0x1705a38de0fe19ad, 0x4d68f8b55a302ede, 0x0c73ddfccb523903, 0xcf1c9ecada08bdb1, 
   0xc07dfa4b1b31e08a, 0x9ff40ac2bef8e1cb, 0x15de36af9c1402c8, 0x77e35e4c77ef98cd, 
   0x16334aef4e87d4bc, 0x8363e88923527415, 0xad7b7f6c407175bd, 0x760a8eba1a375816, 
   0xcb5c67c8ee9bf326, 0x26f57f523d5ecd40, 0x851d28f6568b20da, 0xeb851c09fcc3401b, 
   0x1f394405f75fa05b, 0x25e137bf0ce469db, 0xf40f7636a94cb7c8, 0x7a4e33ccb884dca7, 
   0x2a64fd39a2ea1cde, 0x29441f90567d3f67, 0x5909e590b7c0fcd9, 0x6c6c7698afcb6b60, 
   0xd8341fe34a4e25b6, 0x97f0518df31f147d, 0x4499f756e7999515, 0xe162f0d0186f437f, 
   0xba7295fb0f92a239, 0x36defd251b18293d, 0xbd5d8efd7e5df62f, 0x46727f46e11f2603, 
   ],
  [0x25ce098a3ecaf88f, 0x84a93efe9a64aebe, 0xa2e2d7317b6a0863, 0x1aadf589cb74a4b3, 
   0x4fa9d7c49329013a, 0xbbee07fa4b185db8, 0xa3ef885874280303, 0xcde0d291292af3c5, 
   0xf2abad22f178f762, 0x5fd6047cc8539e0c, 0xc2ff838622853a30, 0x805808c85040bc44, 
   0x81c80e4a9f3fac3c, 0x3f2d3fcc88a55c64, 0x96ee215a8cd78fdc, 0xa623018d525057c3, 
   0xdabf22a2194167ac, 0x5004f4f63d198e63, 0xc1604aab2db245bf, 0x4323fadc0227aef4, 
   0x28db06407f44dca9, 0x17f4458b0bb85f50, 0xc3105e100f8efbee, 0x3fffddfba860dd99, 
   0x968490cee8705a62, 0xea22b2afa6566256, 0xecdce97896ec9b12, 0x49b48359c2cdc6a4, 
   0xd080f1ac92bf7429, 0xdd511cb8cbc02081, 0xecad15540f4fa080, 0x79b29ac6bf2b1fc1, 
   0x19ccaf6ca81699be, 0x7d0c37b491cd1b0d, 0x5b810155ec3fe27b, 0xac0b830b72ae637f, 
   0xacda3ffc034e0079, 0x03262565acb5a239, 0xcda0785430b9673d, 0x94509f736f007ab3, 
   0x985d025b2b9ab686, 0xafaa1087e966e26b, 0xdc815ef3c99fd56f, 0xc5aeff27115e5105, 
   0x08130ca6db060ce7, 0xd987f53624d3d93e, 0xc0f17d58e553f10c, 0x44e629b8559895ee, 
   0x8248221b6f211f45, 0x35c63dc0ea262c3b, 0x4a1093393ab80ea3, 0xeeca95338be6b67e, 
   0xc61bbaa18ab72492, 0x99ec1512dc4f2e5b, 0xbf74cde8827946c7, 0xbabd85f52ee0e58a, 
   0x46c5e6036db706c1, 0xfab44c49176df88c, 0x5b28d620e1d6807d, 0x667318fb21c0923d, 
   0x4d13cffacd523c59, 0xaa7d18e5770eae61, 0xe3a22266bd7d0b7d, 0xf6e43477927c8e2b, 
   ],
  [0x57e2f4c948509df7, 0x5d9890cb9f41a10c, 0x8ed538882eaae14e, 0x43d825eb867e270c, 
   0x78f78bbb3a9e9842, 0x52f19dbad3a1f5f2, 0xf506abca65f78a99, 0x114be3a1ba15e3cc, 
   0x3b702b8bb676bb0b, 0x8182885402aff4bd, 0xe10ae791b7e82414, 0xfd1ff4d688b4715c, 
   0x5c9522ae1beecebe, 0x3911ab6a9cfb7ca3, 0x0e6c30179dbdc29a, 0xb530d56dda9b5f9f, 
   0x0707ccf611ca6ee0, 0x86c1b98e46d4423d, 0x93ff5fabecfcc5f9, 0x141a0f26d8cf6232, 
   0x067bbc572a0c3076, 0x065bb9116a76f596, 0xbf75702c7057bdfd, 0x2ee78a15516bcdc6, 
   0x0aeadb7d0f38abdd, 0xb1f82c8035e38dca, 0x41d083cccc707851, 0xc1e6c58f1f3eace1, 
   0x092dbe4b88c7c9f8, 0xb202a8fba7cd386b, 0x29a7ee39b577d8db, 0xe784d8c6a3a461aa, 
   0x1533a3374c1c03e9, 0xa1d4abd266a5e1ae, 0x41561482d4ff01ed, 0x07c3dfd747f724ae, 
   0xf64ea50fb2ea2167, 0x39422ba2cae14ce5, 0x40f8236d30c3173c, 0x18e3df7e04a4de12, 
   0x6c609365ec04532e, 0x13816fe20cbd5008, 0x96534fcb1ca128a8, 0x0e5a7789063540f9, 
   0x43e8630d59063438, 0xf86d52c1a81e3650, 0x488ad91ed6719e3c, 0xb020d92da07d7200, 
   0x65a3f3ba5be908e4, 0x388ef9b633ea63f5, 0x791e9d633d8fea72, 0x95a955a7370c2926, 
   0xae23f6b94ea4b8f4, 0xbcf332c19b034c85, 0x745b354f27cecb5c, 0x7afc12a590cc01ca, 
   0x1aa320543e46ab49, 0x97f351207a47d45f, 0x1d84534f09a3ac37, 0x44fd7ca5878306af, 
   0xd52586fdda5c4b62, 0xf6abb6e1e97e6b74, 0x48c414682d1ccaa2, 0xcc0dc5a8144e1bb7, 
   ],
  [0x190e9f8280168a40, 0xaf5591e73f543754, 0x5490e17a36b9ca3a, 0xf9ea7320bbbd772e, 
   0x47de0e7bb7f20640, 0x33f4a3dc184c724c, 0xdbe844c7ce79f9a8, 0x6a339d2e2018605b, 
   0xf29b99093afd265b, 0x16edad3e66a42ad7, 0x485e995b58bd3742, 0xa913d6fbdcf1ddac, 
   0xba156d2a86a4d517, 0xac949df992c4397d, 0xcea9577ecb5e4e4f, 0xe39c522c2ab48113, 
   0xa0a2fb13d641ccd1, 0xff8fb0d0ee145b08, 0x0516053722c3b1be, 0xc5609416b8e7995d, 
   0xd955393b1d05a12f, 0x02d382ca9d2801ad, 0xfbf5408277fb9aee, 0xdb81393fd219165b, 
   0xe0ab9c9dac5dd2b7, 0x20962d0a738324d2, 0xff84cbd24f01561b, 0x985f7d2565e7b277, 
   0x3b616726a12800d3, 0x6761f485ae540412, 0x1ed9c2da61bfe89a, 0x312f04dba403f1cb, 
   0xe161c3a6a45ee4e8, 0xa2a17967e4706321, 0xb152e96aec8a6ba7, 0x7a83e6e8bbdd76b0, 
   0x9eb22a32f39b605d, 0xe311a5f6df15e1dd, 0x97d0b73871541d5e, 0x2cf4785492d222e4, 
   0x8d0c819db461bfa1, 0x8ffa085ac9aeb49e, 0xa8f731cbdded0e77, 0xe396f755c38da9ab, 
   0x5272cb114e25b838, 0x8d2666b1a840fd48, 0x274e3a1299e22eb2, 0x023572e48d52066f, 
   0xcc219081e342d9a3, 0x60ad19bcc2a654d0, 0xabca04b380de2c79, 0x7208e7f64919ae61, 
   0xeb3a6ed09cd6257d, 0xdccc242220277677, 0xb935f60e09c2455b, 0xb94b848fc3c9f711, 
   0x5857e9ac11af80bf, 0xfab50bb8f287107e, 0x78f210904315c1de, 0x8b13e573a5625ddd, 
   0x23aa48f4cd484e26, 0x1b9f46f45171973a, 0x6c20e1e02279fa94, 0xc8345a461bd32c61, 
   ],
  [0x83e66d496251b279, 0xf1d17aec0c0caf5d, 0x1a076732c052c143, 0x0d812fd241485401, 
   0x0a42eb8b748de4e9, 0x93bc19113740e560, 0x17b2b0f0a0ef3caa, 0xba66c64d8d528853, 
   0x307a0ec8a0e65983, 0xc883dae2e8e195f0, 0xc35a7727725d2adb, 0x4419f90e8234464e, 
   0xc9fb6cca606c7d42, 0x6c2624b10a6bb538, 0x620e6d9ddc912c52, 0xf5a5607153fd5a8a, 
   0x602dd620602342bf, 0xc87c8958d7904bfa, 0x20fe2a282088d5ed, 0x54bac8e7764d1f06, 
   0xd9787a6f817684b8, 0x153247d15c2047b1, 0xa46800726883cb7e, 0xa6df2a67731fd368, 
   0x840de34cedfc36ce, 0x5409ade9f7b5bd23, 0x9505071a83a3fe99, 0x020dd8d05fc03f24, 
   0x4179eda7e420f65a, 0x6a37f13d7f726136, 0x69e5540a85da0874, 0x5b938202e5952164, 
   0xd99e3b788576d226, 0x8dcf2e991f887602, 0x0229b7a78db6a66e, 0xcd8e0b0e5a8d90a2, 
   0x3348b14e914ee303, 0x6668a6c83fbede05, 0xf6aff5109e4482f9, 0xf236811a957f95bc, 
   0xd275e21d06298597, 0x325123b1204e535c, 0x85647e54aa0598fa, 0x673fd493ebbeed4b, 
   0x79f90e40bad9a251, 0xaed20834f324a555, 0x90f2b1fef5ff195f, 0x2c7fdc6d35a350ad, 
   0xea04443fd7919d4c, 0x11264650a51c6d76, 0xccf217915ebbc82c, 0xf7616bdcc28934bc, 
   0x577b39c7ec7c89d5, 0xb7a298b9a6b0a586, 0x6b121dffa7fe654d, 0x4d5865d0be0f6ac7, 
   0x2c8b55173a5525d2, 0x84d7aa9c53e0bb19, 0x202420b4bc549c2f, 0x01da754dee20ded7, 
   0x11dce33b7345caaa, 0x783221403afaabbf, 0x18c4b0f7f3025128, 0x7bd2d6a0a9411f8a, 
   ],
  [0xbd5bf945e5cf3f35, 0x8518572c623bcc0b, 0xcf59bbc67f2bd953, 0x9447b2d8364bba21, 
   0x782a4a6269a78515, 0xa7b4a3366a892f5f, 0xa6c2c943947e0ac1, 0x2362f5a300a46694, 
   0x0c14fe40f5dc0b59, 0x208e5d860840450f, 0x2986d877d06a6645, 0x9711e1096e0983c9, 
   0xf0c530c94bd0428e, 0x1571eb8d2610818b, 0xb7f31f4aecdda248, 0x391b821fec3d8b0c, 
   0xd70414aa0bd2b31a, 0xbbcd8d6edcf290fc, 0x26ce49bcfa4acadd, 0xa496b5f6d8474808, 
   0x0e8ba46a4414c498, 0xb890fb4c9b1ccb11, 0x9348220b71f416c7, 0x2b7c6f38c69eaf2a, 
   0x25dfcbbef4b518f4, 0x57154b0b988ecfc5, 0x5f5c3b8373c0831a, 0xc33d27999c5c849c, 
   0x7775d9971dfc9a95, 0xa5660b3a127366e0, 0xf3046742e608382d, 0xb8fa2e6be0f5831d, 
   0x856e48bfa4810f5b, 0xd24efb96ad6abb85, 0xa1e94dbbd28a2fb2, 0x9e440f206c88976d, 
   0x9af77b87e69ffd03, 0x65d9ea41757fe239, 0x96ee4f323350b3bd, 0x6d2333902e595d9a, 
   0x2328f3f420a3c9a1, 0x8b33e94615c0c672, 0xcca4ff80c7e89b7e, 0xce9410ef84be55c8, 
   0xab618e8e085d5b63, 0xc5b07ff41a5d5915, 0x0957fd7cc3d306e7, 0xdc8cc95f5c1df92a, 
   0x9c1ed964ca4ec858, 0x0ee6b66bcef343c8, 0xe9ce03a46359574f, 0xdc4854977b61d621, 
   0xca6e60c1ccbcbdcd, 0x6db35a82d85f17f6, 0x89fa962300669459, 0xcd0f9153532a6f31, 
   0x5edb5523ddafd7af, 0x9581c6e20c9483c0, 0xd4fbc7d4c85b5ac7, 0x159d64e96721bdb2, 
   0x2948dcf0eea4d249, 0xebff14ae6f5e29ad, 0x10964e899c435ebe, 0x3f021b0c9b7db783, 
   ],
  [0xe27c98de738ecc2c, 0x418072bc40375ab4, 0xe5d7e921ec351901, 0x7ede3018276d1373, 
   0xa494528e85237595, 0x3cc575a3523af63e, 0x68e274125c40cc52, 0xd9e61d6743d5194a, 
   0xdb91d68c98cda1f8, 0x6abec8488fdd44bb, 0xded0340920e0b0f2, 0xbd8b8b1306b50d13, 
   0xbd5460a0abcb43c9, 0xdb4615587e29070e, 0x988e0842c9755680, 0xe2a82b0a83381c2d, 
   0x35305ee2d516c755, 0x8600c787b2d19f9b, 0x3cb1a5ff4ee8ba64, 0xa96a6a94f9377210, 
   0xed4262ee42b398f4, 0x8b871671c44442c5, 0x10d2c1ad0884db00, 0x148f7ffacdaf12b6, 
   0x7681a54ff3802113, 0x55bf1b13836f0591, 0x14e1ef9da0ae9fda, 0xd73eb8598b797465, 
   0xc47c7358b7243fcd, 0xdc6dda0f015824f2, 0xa342f9cb9dbbe83b, 0x07534f93fb2bf12b, 
   0x8b52a29e70168c09, 0x02345fec9e768351, 0x37ae4abb3eae9a17, 0x416cbf5b75f1a2e3, 
   0x6ba7c81bb9e29966, 0x7f32bc6e2eb3c5d4, 0x26784a01881b575f, 0x388fc59e4b575631, 
   0x2866e549e16854dd, 0x788a029e044c1334, 0xdb8f23de2502c612, 0x22ea0cc68f07071e, 
   0x128c7eea3e1b0e90, 0xaadd81b6346c9c8d, 0x39b530d05d5ac6f4, 0x548b82136f7d4e79, 
   0xdcc6025d2a750aa5, 0x640fe4327fd7e9f0, 0xe8869f4767aa3a23, 0x65b2260a49818697, 
   0x32f992a78d15c05a, 0x732a472a45c0bd5a, 0x18a27c13238f2aa0, 0xe72a5eff3abc9cf5, 
   0x081ba23b0968788c, 0x89842e7a3c8f0b7d, 0xf223eb8ede0d6a2b, 0xa6134f72d9d09a3f, 
   0x12a25864f29454cb, 0x6e69b24049b92ea2, 0xa892fd6aea4bb47f, 0x481668986253dcd7, 
   ],
  [0x583815eee0a6aeae, 0xdee3405ad1ff8873, 0x23261f31fdf60ba3, 0xa09de538fc136f74, 
   0x029d46f3cd8a14aa, 0xd96fad3ede076a03, 0xe0b67850c70f8417, 0x1f03b3b4a85c7edf, 
   0xf0437f637a0c91e2, 0x029515c3cd62b041, 0xcc03be0fce9e4561, 0x3c12bf2fbd6801ab, 
   0x2707d65c0ef7db68, 0x422c2fce2dcdeede, 0x451cbfcf7a300283, 0xe7104b012c18a246, 
   0xf312f61eb924c282, 0xaa7a291482386b87, 0xfec4588d1d761a90, 0xd49563e53cd252a5, 
   0x57f7072076ebff40, 0xe9a0c9cd026df8be, 0x47df2352259ecccf, 0xb8ae72d07ad32296, 
   0x5cfb8008c81ae153, 0xf4d06a0843bae051, 0x791583a4a22da1d1, 0x636e7db9899faed2, 
   0x51a62f69a2a3650f, 0x8f74e636b5948005, 0xba5f0f5b3d550845, 0x5b43feea594f1315, 
   0x3587f01fecbde873, 0xc13587b29fd6ef63, 0xd6e1f69d2644fa7d, 0x840012038beb4cc9, 
   0x403cdcd50f968810, 0xb9aa138ed329a100, 0x06dc1ff75443b242, 0xe503080a1703e5a6, 
   0x19f530bf28ba4bed, 0x9e9b720bf970643a, 0x8344c5062af8ca72, 0x8b60cbfa4f1de766, 
   0xe9003d304a46c528, 0x1f5bbf9c668202a2, 0xae2c1e1d62d5ee27, 0xf7f2cc7d08d92c3b, 
   0x805855b50042f0a3, 0xdb5761825fee3d82, 0xff5f2af6e59cc179, 0xf1661b38fa3e86bb, 
   0x3e8aaf0fbe503576, 0x7755e96a22b7089c, 0x732128213939c18d, 0xdcb95f0cef96714e, 
   0x1041c88241a4ce2c, 0x6e6c4814acb18cb0, 0xe6d330021f4fc3a9, 0x2bcab4e51c28966e, 
   0x0b3126bff77c9cc2, 0x04789dbaeedb157b, 0xa822c138744e9d91, 0x79b84d484d6744ea, 
   ],
  [0xbc2bbf0e8d77f0e3, 0x3ea79d67bc0947c2, 0x34f7cf829242e5cd, 0xe97f243c3deab346, 
   0xa86f806c861577a0, 0xf1cc3dee1e2eb54f, 0x4d3223fb28dc5be5, 0xb18519a216c3ba12, 
   0x610d1349668ef9e6, 0x2d2de66533eb5d95, 0x95f2534af4354e6e, 0xa98bb1513ba77f72, 
   0x740deacac5f9ad0c, 0xfee42c6021149d56, 0x8bba97a4740321ec, 0xd2d0352be140dcaf, 
   0x92fe59d0d08c96d6, 0xcffa4ae33424fa7b, 0x47d5a845fea34d4f, 0xa845c5f7c40eb815, 
   0x86c33558ff600025, 0x146bd7d544506965, 0xd50fb27d960030f7, 0x227f56c13192ed56, 
   0xb5132bd230f85d0c, 0xafe4fd487c436eb6, 0xe2664a489a6085eb, 0x241400263f885e20, 
   0x69e9d211171cc69c, 0x1d2a72b2daf2d811, 0xf0a460596b3119e1, 0x5b1c7e9f71b92392, 
   0xca740548efcaf555, 0x0f490685f9fc95ff, 0x3a54b5e2f9bdf314, 0xdbb0e1a176b4cdf2, 
   0x4bd9c740310698e9, 0x3613e55d3cef8f98, 0x2ea7ab482188b687, 0xbe36b84a0962e62a, 
   0x64c4b48d4bc0e966, 0x9c2ccf4d5e29f134, 0xd94c0fdfaf636fd6, 0x7389dffe9c0e9c89, 
   0x88298266b6e083f0, 0x853239b4ae3dbef7, 0x89d75a8acf44984a, 0x7c8a3e8e1f0143fa, 
   0xae049cb514eafffd, 0x09412d52f1077542, 0x679b52b42dc06b28, 0x94923d6a7b1821fe, 
   0xcb43d75e31a3ae3d, 0x325a60911caec621, 0xd9b14846aa3833ed, 0x2d2fb0019e8918ca, 
   0x6337e501ec1d787b, 0x70ce781b3f023c4b, 0xb53b99301b9c4af3, 0x857f8ab848abaea6, 
   0xd1941ca147f3907b, 0x3acb231998a1099b, 0xde4e05ef38714d48, 0x1487a2d3285ffdd2, 
   ],
  [0x19b2b0c38b1876a9, 0x67721dae5ccecc04, 0x97b98d124b7cd09c, 0x40bda66e36b78e3d, 
   0xf56bed842a3cce61, 0x35eeb73c2bed0fbd, 0xa28532383cca9ae2, 0x675feb59c3e7f757, 
   0xee06546d6d850d79, 0xf89e014a85f47923, 0x4669fadc90e7b911, 0x61e3c205c34c1223, 
   0x60a10f8c5c0462c7, 0x0bdbb6fcbe783bc2, 0x2f1d959528187b35, 0x417a789746362683, 
   0x8c9602b635ff8a0e, 0xc75aebefba03369a, 0x3e3e54452f87d677, 0xcdf2e191f6d5a41b, 
   0x2d08f75107f46993, 0x1784b9a7ded38580, 0xc4438887d029428b, 0x1cf62f765814d2ce, 
   0x73d2822f3a1f88eb, 0xb49c6e8fc8e0ed3a, 0x2046e966b62d2260, 0x2b284357940de024, 
   0x4a6f3c07619172e4, 0xb39e4531f7506901, 0x6d9690251ea2a993, 0x91c045997907e800, 
   0x8098627338afe7d0, 0xdf17bad691e34c1f, 0xcc6ba91bf71e6d47, 0x3495d3d00a79e05d, 
   0x95861d0741a4a4dd, 0x2739ddf3473d7ad4, 0x68433910abf53521, 0xd3124ca1bf5db70b, 
   0x74ca4ce3c0e5aea5, 0x0c018f2052ab6cb8, 0xa161e4fbfd801461, 0x503fb79930e02c09, 
   0xb6cd76b1a329ce11, 0xf08913ed404439cd, 0x76754b74c90f62fe, 0x1546fbc5886c03bb, 
   0xa711064b32f3557c, 0x277d350dab406cd6, 0xf215e24b5e89198f, 0x2fd2a20e59bb887c, 
   0xe4b2fa219a3d606c, 0xf368d04df90481ac, 0xb3d4ca7574bd63af, 0x8a996ad9382912a8, 
   0x34028dd67752ea8d, 0x02079714ca674c18, 0x798de3039aa38480, 0x5288c0b882f5e389, 
   0x94850ab466e5ca0e, 0x77703d528a2f421a, 0xc1a5419ef894da1e, 0x0e2745901344b42a, 
   ],
  [0x3db684e39b4e77c2, 0xa83a205ff6475bad, 0x97bddb3b0fbc9e5a, 0x33f10527668331e6, 
   0x973a9f59f2b48e05, 0xc1c2f928f100c659, 0xce57d8f61b327195, 0x266d62f2d5c6d036, 
   0x37526985bbf39016, 0x4c1a67835b0eb2b6, 0xb11d487ecce4c253, 0x71a194ce6e29ff17, 
   0x10cca768553a8929, 0x5800dabbb6e1967b, 0x45af95721c08ab97, 0x1cd20b8f9168bf00, 
   0x21bb395a9f79ae7e, 0xd2dcc9a6c4a702a2, 0xc95f406508f4a427, 0xac6390282216a4af, 
   0x6d5e1f9ea7224226, 0x8e28a59c84fc4f22, 0x9d7c156d757a3f3c, 0x2e857544be7c8442, 
   0xf3b1c81c633d2dca, 0x8ac142e1fb2b6511, 0x2d277a333190579b, 0x7f97ac12dafdf1f4, 
   0x324620b61024f717, 0x715394c0dda99b39, 0x331a124d498d264f, 0x49a535368b184fc3, 
   0x29e7b056f6ddef9c, 0xb6f21720373e3343, 0x8e36c08d817effe5, 0x2a0d0a3dac8b5b5a, 
   0xdffb160c28b7db6c, 0x70392ac739c9efda, 0x4900fe106d30ca85, 0xbacd47404c28aa50, 
   0x1ab0a3f80dd86f04, 0x74dd3176bb8c9eda, 0xd0c04dc5c4c97167, 0xd1a6a2d6e7ba5a45, 
   0x084c598bd0c91fb1, 0x31d07467554541ce, 0x70c6e2fe931c78cd, 0xa8384e8780c5d37c, 
   0x7f6f665c0b6b07da, 0xcf7dbc71c7ed6755, 0xbf088426e267c8db, 0x2f66de4e6c5e518a, 
   0x6e8217a5b271cfa9, 0xb57ef58f907fe676, 0x5806415fe41c443a, 0x5c424e23b0263232, 
   0x7d03454e5d20b2d6, 0xa26ee42e938a8633, 0xbe7019a783347fff, 0x7c1b7db69f5208c0, 
   0xf98a3aed5e549cd1, 0xc3a13aafc10521a8, 0xb329dcfef82c41eb, 0x565e621e81a103a8, 
   ],
  [0x89b46061a3c9abaf, 0x4cd26b92c1e13338, 0xe630075bc4f53f5e, 0x75631a8b6c98c2a0, 
   0x31f29bbcf26f5b25, 0x7db5f989229fecd3, 0x6bc30116652f87ca, 0x17a7ebd226776a45, 
   0xc38df111990bb5d1, 0x463d8169639af6fa, 0xef7c9e3e72113d97, 0x31fb7ccd99509a2c, 
   0x4a2e807a25313018, 0x95bef13d13c45478, 0x88c8451097d9c2d0, 0x87e272fdf276c555, 
   0xa67a2e39f7b7dd9f, 0x37a74ed6f9d435e4, 0x0a5e25e36e88648f, 0xd63b811969d1b786, 
   0x9da2ae45734d3569, 0x61a3ccbb26cdd543, 0x9a607388a50f7e29, 0x1ef775588d75ca95, 
   0x49c36ca190cf2440, 0xc415f95a50bfaa08, 0xe79eb341900d575c, 0xe91019648f771af2, 
   0x08de0079687f0847, 0xd71847e66b4b0797, 0x596f68c73d1e9458, 0x75eb2cef58aabf70, 
   0x44bddcc19746360e, 0xb531a3bcf0e0f9e2, 0xe3f3edddaf210091, 0x0e6730d3718d46d0, 
   0xa1139d94cc5fe85d, 0x5f2a587aceace0cd, 0x8355538c58882a4c, 0x0b2e5fd8354a2918, 
   0x26d3d6d0641dfb4a, 0xcf7f3063d8ba523c, 0x72e55a5228e82998, 0x5aff26708c169fcb, 
   0x1476233dfb382034, 0xaf752ae3f028dd74, 0x06a34aed85cd51ce, 0x606008f6767178ed, 
   0x16d468082b66015c, 0x159e828816271a5e, 0xfadcc6406b5203ea, 0xa7a81c4e45a31d6d, 
   0x8bce33d6926ca017, 0x5d9fe89d094c9c97, 0x0382b6f07337c06e, 0x56394dbf4ef0746e, 
   0x0d22bcd31cf14f06, 0x07eaf004f74bac18, 0x2d3909aa234d3d40, 0xa6a45281ed98f73a, 
   0x2374aade06314854, 0xdedb0a0760edbfc4, 0x1d4914a0c9cd15bb, 0x50a604db5b0c8150, 
   ],
];
pub const SIDE_KEY: u64 = 0xbbd668042614cc52;
pub const CASTLING_KEYS: [u64; 4] = [
  0xf40524dc5075b14a,  0x772114afd2c048a8,  0x5b4befeff010e239,  0xdd6712d0ecf56dbd,
];
pub const EN_PASSANT_FILE_KEYS: [u64; 8] = [
  0xd5e2ced04d8c4826,  0xe0feb8ec356d9936,  0xc7f6f2b29a4fcf84,  0xd8e2ca48f74e311b,  0x86b86517647d8bb9,  0xf21975ca997cbeee,  0x1048efd0bc9a3c13,  0x36cf646fa5b86cab,
];

// Exact: every move at this node was searched to completion without a cutoff, so `score`
// is the position's real, true value -- safe to reuse outright.
//
// LowerBound (fail-high): a beta cutoff happened (alpha >= beta) before every move was
// checked, because some move was already too good for the opponent to ever allow this
// position to be reached. We stopped looking, so we never found out exactly how good --
// all we know is the true value is *at least* `score`.
//
// UpperBound (fail-low): none of this node's moves managed to beat the alpha the caller
// came in with, so nothing here is better than what the caller could already guarantee
// elsewhere. All we know is the true value is *at most* `score`.
pub enum Bound {
    Exact,
    LowerBound,
    UpperBound,
}

pub struct TTEntry {
    // The depth to which we have already explored the position (it has to be >= of the depth we want to explore)
    pub depth: u32,
    pub bound: Bound,
    // The score obtained
    pub score: i32,
    // The move done
    pub best_move: Move,
}

impl Board{
    pub fn calculate_zobric_hash(&self) -> u64{
        let mut result = 0u64;

        // First hash each pawn
        // WHITE
        result = self.calculte_hash_piece_square_keys(WHITE_PAWN, result);

        // BLACK
        result = self.calculte_hash_piece_square_keys(BLACK_PAWN, result);

        // Knights
        // WHITE
        result = self.calculte_hash_piece_square_keys(WHITE_KNIGHT, result);

        //BLACK
        result = self.calculte_hash_piece_square_keys(BLACK_KNIGHT, result);

        // Bishops
        // WHITE
        result = self.calculte_hash_piece_square_keys(WHITE_BISHOP, result);

        //BLACK
        result = self.calculte_hash_piece_square_keys(BLACK_BISHOP, result);

        // Rooks
        // WHITE
        result = self.calculte_hash_piece_square_keys(WHITE_ROOK, result);

        //BLACK
        result = self.calculte_hash_piece_square_keys(BLACK_ROOK, result);

        // Queens
        // WHITE
        result = self.calculte_hash_piece_square_keys(WHITE_QUEEN, result);

        //BLACK
        result = self.calculte_hash_piece_square_keys(BLACK_QUEEN, result);

        // Kings
        // WHITE
        result = self.calculte_hash_piece_square_keys(WHITE_KING, result);

        //BLACK
        result = self.calculte_hash_piece_square_keys(BLACK_KING, result);

        // Side to move
        if self.side_to_move == Black {
          result ^= SIDE_KEY;
        }

        // Castling rights
        let mut castling_rights = self.castling_rights.clone();

        while castling_rights != 0 {
          let index = castling_rights.trailing_zeros();
          result ^= CASTLING_KEYS[index as usize];

          castling_rights &= castling_rights - 1;    
        }

        // En_passant: a single file (or none active), not a set of flags -- no bit-scan needed.
        if self.en_passant_square != NO_SQUARE {
          result ^= EN_PASSANT_FILE_KEYS[(self.en_passant_square % 8) as usize];
        }

        result
    }

    fn calculte_hash_piece_square_keys(&self, piece: u8, mut result: u64) -> u64{
      let index = piece - 1;
      let mut bitboard = self.piece_bitboards[index as usize];

      while bitboard != 0{
          let square_index = bitboard.trailing_zeros();
          result ^= PIECE_SQUARE_KEYS[index as usize][square_index as usize];

          bitboard &= bitboard - 1;
        }

        result
    }

    pub fn update_zobrian_hash(&mut self, action: &Action) {
      // The piece that actually ends up on destination -- the moved piece, unless this move
      // is a promotion, in which case the pawn disappears and the promoted piece appears.
      let placed_piece = action.mv.promotion.unwrap_or(action.moved_piece);

      // Detect castling the same way make_move does: a king move that jumps 2 squares also
      // relocates the rook, derived from the king's own destination square.
      let is_king = action.moved_piece == WHITE_KING || action.moved_piece == BLACK_KING;
      let rook_move: Option<(u8, u8)> = if is_king && (action.mv.origin as i8 - action.mv.destination as i8).abs() == 2 {
        Some(match action.mv.destination {
          2 => (0, 3),
          6 => (7, 5),
          58 => (56, 59),
          62 => (63, 61),
          _ => panic!("king move of 2 squares to an unknown castle destination: {}", action.mv.destination),
        })
      } else {
        None
      };

      // Mover: out of origin, into destination (using placed_piece so promotion is handled).
      self.zobrian_hash ^= PIECE_SQUARE_KEYS[(action.moved_piece - 1) as usize][action.mv.origin as usize];
      self.zobrian_hash ^= PIECE_SQUARE_KEYS[(placed_piece - 1) as usize][action.mv.destination as usize];

      // Captured piece (if any): removed from its actual square, which differs from
      // destination for en passant.
      if let (Some(captured_piece), Some(captured_square)) = (action.captured_piece, action.captured_square) {
        self.zobrian_hash ^= PIECE_SQUARE_KEYS[(captured_piece - 1) as usize][captured_square as usize];
      }

      // Castling: the rook also relocates, out of its own origin and into its own destination.
      if let Some((rook_origin, rook_destination)) = rook_move {
        let rook_piece = if action.moved_piece == WHITE_KING { WHITE_ROOK } else { BLACK_ROOK };
        self.zobrian_hash ^= PIECE_SQUARE_KEYS[(rook_piece - 1) as usize][rook_origin as usize];
        self.zobrian_hash ^= PIECE_SQUARE_KEYS[(rook_piece - 1) as usize][rook_destination as usize];
      }

      // Side to move: flips every move, so this XOR is unconditional, not gated on the new value.
      self.zobrian_hash ^= SIDE_KEY;

      // Castling rights: only the bits that actually changed need toggling, not every bit
      // that's currently set (a^b isolates exactly the differing bits between old and new).
      let mut changed_castling_bits = self.castling_rights ^ action.previous_castling_rights;
      while changed_castling_bits != 0 {
        let index = changed_castling_bits.trailing_zeros();
        self.zobrian_hash ^= CASTLING_KEYS[index as usize];

        changed_castling_bits &= changed_castling_bits - 1;
      }

      // En passant: remove the old file's key if one was active, and separately add the new
      // one if one is now active -- these are independent, not mutually exclusive.
      if action.previous_en_passant_square != NO_SQUARE {
        self.zobrian_hash ^= EN_PASSANT_FILE_KEYS[(action.previous_en_passant_square % 8) as usize];
      }
      if self.en_passant_square != NO_SQUARE {
        self.zobrian_hash ^= EN_PASSANT_FILE_KEYS[(self.en_passant_square % 8) as usize];
      }
    }
}