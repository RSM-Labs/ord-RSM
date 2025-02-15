use super::*;

pub(super) struct Message {
  pub(super) flaw: Option<Flaw>,
  pub(super) edicts: Vec<Edict>,
  pub(super) mint20_edicts: Option<Vec<Edict>>,
  pub(super) mint21_edicts: Option<Vec<Edict>>,
  pub(super) mint30_edicts: Option<Vec<Edict>>,
  pub(super) mint31_edicts: Option<Vec<Edict>>,
  pub(super) burn2_edicts: Option<Vec<Edict>>,
  pub(super) burn3_edicts: Option<Vec<Edict>>,
  pub(super) fields: HashMap<u128, VecDeque<u128>>,
}

impl Message {
  pub(super) fn from_integers(tx: &Transaction, payload: &[u128]) -> Self {
    let mut edicts = Vec::new();
    let mut mint20_edicts = Vec::new();
    let mut mint21_edicts = Vec::new();
    let mut mint30_edicts = Vec::new();
    let mut mint31_edicts = Vec::new();
    let mut burn2_edicts = Vec::new();
    let mut burn3_edicts = Vec::new();
    let mut fields = HashMap::<u128, VecDeque<u128>>::new();
    let mut flaw = None;

    for i in (0..payload.len()).step_by(2) {
      let tag = payload[i];

      if Tag::Body == tag {
        let mut id = RuneId::default();
        for chunk in payload[i + 1..].chunks(4) {
          if chunk.len() != 4 {
            flaw.get_or_insert(Flaw::TrailingIntegers);
            break;
          }

          let Some(next) = id.next(chunk[0], chunk[1]) else {
            flaw.get_or_insert(Flaw::EdictRuneId);
            break;
          };

          let Some(edict) = Edict::from_integers(tx, next, chunk[2], chunk[3]) else {
            flaw.get_or_insert(Flaw::EdictOutput);
            break;
          };

          id = next;
          edicts.push(edict);
        }
        break;
      }

      if Tag::Mint20Body == tag {
        let mut id = RuneId::default();
        for chunk in payload[i + 1..].chunks(4) {
          if chunk.len() != 4 {
            flaw.get_or_insert(Flaw::TrailingIntegers);
            break;
          }

          let Some(next) = id.next(chunk[0], chunk[1]) else {
            flaw.get_or_insert(Flaw::EdictRuneId);
            break;
          };

          let Some(edict) = Edict::from_integers(tx, next, chunk[2], chunk[3]) else {
            flaw.get_or_insert(Flaw::EdictOutput);
            break;
          };

          id = next;
          mint20_edicts.push(edict);
        }
        break;
      }

      if Tag::Mint21Body == tag {
        let mut id = RuneId::default();
        for chunk in payload[i + 1..].chunks(4) {
          if chunk.len() != 4 {
            flaw.get_or_insert(Flaw::TrailingIntegers);
            break;
          }

          let Some(next) = id.next(chunk[0], chunk[1]) else {
            flaw.get_or_insert(Flaw::EdictRuneId);
            break;
          };

          let Some(edict) = Edict::from_integers(tx, next, chunk[2], chunk[3]) else {
            flaw.get_or_insert(Flaw::EdictOutput);
            break;
          };

          id = next;
          mint21_edicts.push(edict);
        }
        break;
      }

      if Tag::Mint30Body == tag {
        let mut id = RuneId::default();
        for chunk in payload[i + 1..].chunks(4) {
          if chunk.len() != 4 {
            flaw.get_or_insert(Flaw::TrailingIntegers);
            break;
          }

          let Some(next) = id.next(chunk[0], chunk[1]) else {
            flaw.get_or_insert(Flaw::EdictRuneId);
            break;
          };

          let Some(edict) = Edict::from_integers(tx, next, chunk[2], chunk[3]) else {
            flaw.get_or_insert(Flaw::EdictOutput);
            break;
          };

          id = next;
          mint30_edicts.push(edict);
        }
        break;
      }

      if Tag::Mint31Body == tag {
        let mut id = RuneId::default();
        for chunk in payload[i + 1..].chunks(4) {
          if chunk.len() != 4 {
            flaw.get_or_insert(Flaw::TrailingIntegers);
            break;
          }

          let Some(next) = id.next(chunk[0], chunk[1]) else {
            flaw.get_or_insert(Flaw::EdictRuneId);
            break;
          };

          let Some(edict) = Edict::from_integers(tx, next, chunk[2], chunk[3]) else {
            flaw.get_or_insert(Flaw::EdictOutput);
            break;
          };

          id = next;
          mint31_edicts.push(edict);
        }
        break;
      }

      if Tag::Burn2Body == tag {
        let mut id = RuneId::default();
        for chunk in payload[i + 1..].chunks(4) {
          if chunk.len() != 4 {
            flaw.get_or_insert(Flaw::TrailingIntegers);
            break;
          }

          let Some(next) = id.next(chunk[0], chunk[1]) else {
            flaw.get_or_insert(Flaw::EdictRuneId);
            break;
          };

          let Some(edict) = Edict::from_integers(tx, next, chunk[2], chunk[3]) else {
            flaw.get_or_insert(Flaw::EdictOutput);
            break;
          };

          id = next;
          burn2_edicts.push(edict);
        }
        break;
      }

      if Tag::Burn3Body == tag {
        let mut id = RuneId::default();
        for chunk in payload[i + 1..].chunks(4) {
          if chunk.len() != 4 {
            flaw.get_or_insert(Flaw::TrailingIntegers);
            break;
          }

          let Some(next) = id.next(chunk[0], chunk[1]) else {
            flaw.get_or_insert(Flaw::EdictRuneId);
            break;
          };

          let Some(edict) = Edict::from_integers(tx, next, chunk[2], chunk[3]) else {
            flaw.get_or_insert(Flaw::EdictOutput);
            break;
          };

          id = next;
          burn3_edicts.push(edict);
        }
        break;
      }

      let Some(&value) = payload.get(i + 1) else {
        flaw.get_or_insert(Flaw::TruncatedField);
        break;
      };

      fields.entry(tag).or_default().push_back(value);
    }

    Self {
      flaw,
      edicts,
      mint20_edicts: Some(mint20_edicts),
      mint21_edicts: Some(mint21_edicts),
      mint30_edicts: Some(mint30_edicts),
      mint31_edicts: Some(mint31_edicts),
      burn2_edicts: Some(burn2_edicts),
      burn3_edicts: Some(burn3_edicts),
      fields,
    }
  }
}
