use crate::classifiers::classifier_query::ClassifierQuery;
use crate::classifiers::classifier_response::ClassifierResponse;

pub trait Classifier <L: ToString, V: ToString> {
    fn classify(&self, query: ClassifierQuery) -> Result<ClassifierResponse<L,V>,()>;
}