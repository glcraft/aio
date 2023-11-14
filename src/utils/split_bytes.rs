use bytes::Bytes;

pub struct SplitBytesFactory<Sep> 
where 
    Sep: AsRef<[u8]>
{
    separator: Sep,
    rest: Vec<u8>,
}

impl<Sep> SplitBytesFactory<Sep> 
where
    Sep: AsRef<[u8]> + Clone
{
    pub fn new(separator: Sep) -> Self {
        Self {
            separator,
            rest: Vec::new(),
        }
    }
    pub fn new_iter(&mut self, bytes: Bytes) -> SplitBytes<Sep> {
        let sep_len = self.separator.as_ref().len();
        let pos_last_separator_found = bytes
            .windows(sep_len)
            .rev()
            .position(|b| b == self.separator.as_ref())
            .map(|i| i + sep_len);

        unsafe {
            println!("total bytes: {}", String::from_utf8_unchecked(bytes.to_vec()));
        }

        // si none, la trame n'est pas complete
        // ne 
        return match pos_last_separator_found {
            Some(pos_last_separator) => {
                let pos_last_separator = bytes.len() - pos_last_separator;
                unsafe {
                    println!("pos_last_separator: {}", String::from_utf8_unchecked(bytes.slice(..pos_last_separator).to_vec()));
                }
                let mut current = Vec::new();
                std::mem::swap(&mut current, &mut self.rest);
                current.append(&mut bytes.slice(..pos_last_separator).to_vec());
                self.rest = bytes.slice((pos_last_separator + sep_len).min(bytes.len())..).to_vec();
                SplitBytes::new(bytes.slice(..pos_last_separator), self.separator.clone())
            }
            None => {
                self.rest.append(&mut bytes.to_vec());
                SplitBytes::new(Bytes::new(), self.separator.clone())
            }
        };

        if let Some(pos_last_separator) = pos_last_separator_found { 
            let mut current = Vec::new();
            std::mem::swap(&mut current, &mut self.rest);
            current.append(&mut bytes.slice(..pos_last_separator).to_vec());
            self.rest = bytes.slice((pos_last_separator + sep_len).min(bytes.len())..).to_vec();
            return SplitBytes::new(bytes.slice(..pos_last_separator), self.separator.clone());
        }
        let pos_last_separator = bytes.len() - pos_last_separator_found.unwrap_or(0);

        
        
        let mut current = Vec::new();
        std::mem::swap(&mut current, &mut self.rest);
        current.append(&mut bytes.slice(..pos_last_separator).to_vec());
        self.rest = bytes.slice((pos_last_separator + sep_len).min(bytes.len())..).to_vec();
        SplitBytes::new(Bytes::from(current), self.separator.clone())
    }
}

pub struct SplitBytes<Sep> 
where 
    Sep: AsRef<[u8]>
{
    bytes: Bytes,
    separator: Sep,
    index: Option<usize>,
}

impl<Sep> SplitBytes<Sep> 
where
    Sep: AsRef<[u8]>
{
    fn new(bytes: Bytes, separator: Sep) -> Self {
        Self {
            bytes,
            separator,
            index: Some(0),
        }
    }
}

impl<Sep> Iterator for SplitBytes<Sep> 
where
    Sep: AsRef<[u8]>
{
    type Item = Bytes;
    fn next(&mut self) -> Option<Self::Item> {
        let separator = self.separator.as_ref();
        let index = self.index?;
        let bytes = self.bytes.slice(index..);
        let found = bytes
            .windows(separator.len())
            .find(|b| b == &separator);
        let slice_bytes = if let Some(found) = found {
            let end_selection = found.as_ptr() as usize - bytes.as_ptr() as usize;
            self.index = self.index.map(|i| i + end_selection + found.len());
            bytes.slice(..end_selection)
        } else {
            self.index = None;
            bytes
        };
        match slice_bytes.is_empty() {
            false => Some(slice_bytes),
            true => None,
        }
    }
}