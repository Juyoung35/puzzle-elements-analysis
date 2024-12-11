pub struct HoppNotation {
    shape_seed: Vec<u8>,
    shape_groups: Vec<Vec<u8>>,
    transforms: Vec<HoppTransform>,
}
static RE_LOGIN: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?x)
    ^(?P<login>[^@\s]+)@
    ([[:word:]]+\.)*
    [[:word:]]+$
    ").unwrap()
);

use regex::Regex;
// (?<middle>.)
static REGEX_TRANSFORM: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"
    (?<action>[mr])
    (?<action_angle>[\d.]*)?
    (?
        (?<point_type>[chv])?
        (?<point_index>\d+)?
    \)?
").unwrap());
// let re = Regex::new(r"(?m)^([^:]+):([0-9]+):(.+)$").unwrap();
// const REGEX_TRANSFORM = /([mr])([\d.]*)?\(?([chv])?(\d+)?\)?/i;
impl HoppNotation {
    pub fn parse(h: Configuration) -> Self {
        let (shapes, transforms) = ...;
        let mut shape_seed = Vec::new();
        let mut shape_groups = Vec::new();
        for (i, shape_group) in shapes.split(DELIMITER_PHASE).expect("").enumerate() {
            if i == 0
            let shape_group_splitted = shape_group.split(DELIMITER_SHAPE).map(|c| c.parse::<u8>()).collect::<Vec<u8>>();
            if i == 0 {
                shape_seed = shape_group_splitted;
            } else {
                shape_groups.push(shape_group_splitted);
            }
        }
        let transform_entities = transforms.map(Self::to_transform);
        Self {
            shape_seed,
            shape_groups,
            transform_entities,
        }
    }

    pub fn to_transform(transform: String) -> HoppTransform {
        // let m = REGEX_TRANSFORM.exec(transform);
        let Some(caps) = re.captures(hay) else { return };
        REGEX_TRANSFORM.captures(transform).and_then("")
    }
}

pub struct HoppTransform {
    
}

const DELIMITER_STAGE = '/';
const DELIMITER_PHASE = '-';
const DELIMITER_SHAPE = ',';



const toTransform = (transform: string): Transform | undefined => {
    const match = REGEX_TRANSFORM.exec(transform);
  
    if (match) {
      const [,
        action,
        actionAngle = '180',
        pointType,
        pointIndex,
      ] = match as unknown as [
        string,
        TypeAction,
        string | undefined,
        TypePointType | undefined,
        string | undefined,
      ];
  
      if ((action === 'm' || action === 'r')) {
        return {
          action: action,
          actionAngle: toRadians(+actionAngle),
          pointIndex: pointIndex ? +pointIndex : 0,
          pointType: pointType,
          string: transform,
        };
      }
    }
  };