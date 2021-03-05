use serde::{de, ser::SerializeStruct, Deserialize, Serialize};
use std::fmt;
use std::marker::PhantomData;

use crate::{traits::Index, util::OptionIndex, Triangulation};

impl<I> Serialize for OptionIndex<I>
where
    I: Serialize + Index,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.get().serialize(serializer)
    }
}

impl<'de, I> Deserialize<'de> for OptionIndex<I>
where
    I: Deserialize<'de> + Index,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let opt = Option::<I>::deserialize(deserializer)?;
        Ok(OptionIndex::from(opt))
    }
}

impl<I> Serialize for Triangulation<I>
where
    I: Serialize + Index,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[cfg(feature = "vertices")]
        const FIELD_COUNT: usize = 4;
        #[cfg(not(feature = "vertices"))]
        const FIELD_COUNT: usize = 3;

        let mut state = serializer.serialize_struct("Triangulation", FIELD_COUNT)?;

        #[cfg(feature = "vertices")]
        state.serialize_field("vertices", &self.vertices)?;

        state.serialize_field("triangles", &self.triangles)?;
        state.serialize_field("halfedges", &self.halfedges)?;
        state.serialize_field("hull", &self.hull)?;

        state.end()
    }
}

impl<'de, I> Deserialize<'de> for Triangulation<I>
where
    I: Deserialize<'de> + Index,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[cfg(feature = "vertices")]
        const FIELDS: &[&str] = &["vertices", "triangles", "halfedges", "hull"];
        #[cfg(not(feature = "vertices"))]
        const FIELDS: &[&str] = &["triangles", "halfedges", "hull"];

        enum Field {
            #[cfg(feature = "vertices")]
            Vertices,
            Triangles,
            Halfedges,
            Hull,
        }

        impl<'de> serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> de::Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        #[cfg(feature = "vertices")]
                        return formatter.write_str("vertices, triangles, halfedges, or hull");
                        #[cfg(not(feature = "vertices"))]
                        return formatter.write_str("triangles, halfedges, or hull");
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            #[cfg(feature = "vertices")]
                            "vertices" => Ok(Field::Vertices),
                            "triangles" => Ok(Field::Triangles),
                            "halfedges" => Ok(Field::Halfedges),
                            "hull" => Ok(Field::Hull),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct TriangulationVisitor<I> {
            phantom: PhantomData<I>,
        };

        impl<'de, I> de::Visitor<'de> for TriangulationVisitor<I>
        where
            I: Deserialize<'de> + Index,
        {
            type Value = Triangulation<I>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("vertices, triangles, halfedges, or hull")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Triangulation<I>, V::Error>
            where
                V: de::SeqAccess<'de>,
            {
                #[cfg(feature = "vertices")]
                let vertices = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let triangles = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let halfedges = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let hull = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;

                Ok(Triangulation {
                    #[cfg(feature = "vertices")]
                    vertices,
                    triangles,
                    halfedges,
                    hull,
                })
            }

            fn visit_map<V>(self, mut map: V) -> Result<Triangulation<I>, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                #[cfg(feature = "vertices")]
                let mut vertices = None;
                let mut triangles = None;
                let mut halfedges = None;
                let mut hull = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        #[cfg(feature = "vertices")]
                        Field::Vertices => {
                            if vertices.is_some() {
                                return Err(de::Error::duplicate_field("vertices"));
                            }
                            vertices = Some(map.next_value()?);
                        }
                        Field::Triangles => {
                            if triangles.is_some() {
                                return Err(de::Error::duplicate_field("triangles"));
                            }
                            triangles = Some(map.next_value()?);
                        }
                        Field::Halfedges => {
                            if halfedges.is_some() {
                                return Err(de::Error::duplicate_field("halfedges"));
                            }
                            halfedges = Some(map.next_value()?);
                        }
                        Field::Hull => {
                            if hull.is_some() {
                                return Err(de::Error::duplicate_field("hull"));
                            }
                            hull = Some(map.next_value()?);
                        }
                    }
                }

                #[cfg(feature = "vertices")]
                let vertices = vertices.ok_or_else(|| de::Error::missing_field("vertices"))?;
                let triangles = triangles.ok_or_else(|| de::Error::missing_field("triangles"))?;
                let halfedges = halfedges.ok_or_else(|| de::Error::missing_field("halfedges"))?;
                let hull = hull.ok_or_else(|| de::Error::missing_field("hull"))?;

                Ok(Triangulation {
                    #[cfg(feature = "vertices")]
                    vertices,
                    triangles,
                    halfedges,
                    hull,
                })
            }
        }

        deserializer.deserialize_struct(
            "Triangulation",
            FIELDS,
            TriangulationVisitor {
                phantom: PhantomData,
            },
        )
    }
}
