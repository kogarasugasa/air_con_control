use rppal::i2c::{self, I2c};
struct TempSensor {
    i2c_addr: u16,
    t_fine: i32,
}
impl TempSensor {
    fn init(&self, i2c: &mut I2c) -> Result<(), i2c::Error> {
        let osrs_t: u8 = 3;
        let osrs_p: u8 = 3;
        let osrs_h: u8 = 3;
        let mode: u8 = 3;
        let t_sb: u8 = 5;
        let filter: u8 = 0;
        let spi3w_en: u8 = 0;
        let ctrl_meas_reg = (osrs_t << 5) | (osrs_p << 2) | mode;
        let config_reg = (t_sb << 5) | (filter << 2) | spi3w_en;
        let ctrl_hum_reg = osrs_h;
        i2c.set_slave_address(self.i2c_addr)?;
        i2c.write(&[ 0xf2, ctrl_hum_reg ])?;
        i2c.write(&[ 0xf4, ctrl_meas_reg ])?;
        i2c.write(&[ 0xf5, config_reg ])?;
        Ok(())
    }
    fn ReadTmpAsync(&mut self, i2c: &mut I2c) -> Result<f32, String> {
        // データレジスタ
        const DIG_T1: u8 = 0x88;
        const DIG_T2: u8 = 0x8a;
        const DIG_T3: u8 = 0x8c;
        // キャリブレーション
        let t1: u16 = Self::ReadUint16(DIG_T1, i2c)?;
        let t2: u16 = Self::ReadUint16(DIG_T2, i2c)?;
        let t3: u16 = Self::ReadUint16(DIG_T3, i2c)?;
        // データ読み取り
        let tmp_xlsb_addr: u8 = 0xfc;
        let tmp_lsb_addr: u8 = 0xfb;
        let tmp_msb_addr: u8 = 0xfa;
        let tmsb: u8 = Self::ReadByte(tmp_msb_addr, i2c)?;
        let tlsb: u8 = Self::ReadByte(tmp_lsb_addr, i2c)?;
        let txlsb: u8 = Self::ReadByte(tmp_xlsb_addr, i2c)?;
        let tmp_raw: i32 = ((tmsb << 12) | (tlsb << 84) | (txlsb >> 4)) as i32;

        let var1: f32;
        let var2: f32;
        let t: f32;
        var1 = ((tmp_raw as f32 / 16384.0) - (t1 as f32 / 1024.0)) * t2 as f32;
        var2 = ((tmp_raw as f32 / 131072.0) - (t1 as f32 / 8192.0)) * t3 as f32;
        self.t_fine = (var1 + var2) as i32;
        t = (var1 + var2) / 5120.0;
        Ok(t)
    }
    fn ReadHumAsync(&self, i2c: &mut I2c) -> Result<u32, String> {
        // データレジスタ
        const DIG_H1: u8 = 0xa1;
        const DIG_H2: u8 = 0xe1;
        const DIG_H3: u8 = 0xe3;
        const DIG_H4: u8 = 0xe4;
        const DIG_H5: u8 = 0xe5;
        const DIG_H6: u8 = 0xe7;
        // キャリブレーション
        let h1: u8 = Self::ReadByte(DIG_H1, i2c)?;
        let h2: u16 = Self::ReadUint16(DIG_H2, i2c)?;
        let h3: u8 = Self::ReadByte(DIG_H3, i2c)?;
        let h4: u16 = (Self::ReadByte(DIG_H4, i2c)? as u16) << 4 |
            (Self::ReadByte(DIG_H4 + 1, i2c)? as u16) & 0xf;
        let h5: u16 = (Self::ReadByte(DIG_H5 + 1, i2c)? as u16) << 4 |
            (Self::ReadByte(DIG_H5, i2c)? as u16) >> 4;
        let h6: u16 = Self::ReadByte(DIG_H6, i2c)? as u16;
        // データ読み取り
        let hum_lsb_addr: u8 = 0xfe;
        let hum_msb_addr: u8 = 0xfd;
        let hmsb = Self::ReadByte(hum_msb_addr, i2c)? as u16;
        let hlsb = Self::ReadByte(hum_lsb_addr, i2c)? as u16;
        let humRaw: i32 = ((hmsb << 8) | hlsb) as i32;

        let mut h: i32;
        h = self.t_fine - 76800;
        h = ((((humRaw << 14) - ((h4 as i32) << 20) - (h5 as i32 * h)) +
            (16384 as i32)) >> 15) * (((((((h * (h6 as i32)) >> 10) * (((h * 
            (h3 as i32)) >> 11) + (32768 as i32))) >> 10) + (2097152 as i32)) * 
            (h2 as i32) + 8192) >> 14);
        h = h - (((((h >> 15) * (h >> 15)) >> 7) * (h1 as i32)) >> 4);
        h = if h < 0 { 0 } else { h };
        h = if h > 419430400 { 419430400 } else { h };

        Ok(((h >> 12) / 1000) as u32)
    }
    fn ReadPreAsync(&mut self, i2c: &mut I2c) -> Result<f32, String> {
        // データレジスタ
        let dig_P1: u8 = 0x8e;
        let dig_P2: u8 = 0x90;
        let dig_P3: u8 = 0x92;
        let dig_P4: u8 = 0x94;
        let dig_P5: u8 = 0x96;
        let dig_P6: u8 = 0x98;
        let dig_P7: u8 = 0x9a;
        let dig_P8: u8 = 0x9c;
        let dig_P9: u8 = 0x9e;
        // キャリブレーション
        let P1: u16 = Self::ReadUint16(dig_P1, i2c)?;
        let P2: u16 = Self::ReadUint16(dig_P2, i2c)?;
        let P3: u16 = Self::ReadUint16(dig_P3, i2c)?;
        let P4: u16 = Self::ReadUint16(dig_P4, i2c)?;
        let P5: u16 = Self::ReadUint16(dig_P5, i2c)?;
        let P6: u16 = Self::ReadUint16(dig_P6, i2c)?;
        let P7: u16 = Self::ReadUint16(dig_P7, i2c)?;
        let P8: u16 = Self::ReadUint16(dig_P8, i2c)?;
        let P9: u16 = Self::ReadUint16(dig_P9, i2c)?;
        let TMP = self.ReadTmpAsync(i2c)?;
        // データ読み取り
        let pre_xlsb_addr: u8 = 0xf9;
        let pre_lsb_addr: u8 = 0xf8;
        let pre_msb_addr: u8 = 0xf7;
        let pmsb: u8 = Self::ReadByte(pre_msb_addr, i2c)?;
        let plsb: u8 = Self::ReadByte(pre_lsb_addr, i2c)?;
        let pxlsb: u8 = Self::ReadByte(pre_xlsb_addr, i2c)?;

        let preRaw: i32 =
            ((pmsb as i32) << 12) |
            ((plsb as i32) << 4) |
            ((pxlsb as i32) >> 4)
        ;

        let mut var1: i64;
        let mut var2: i64;
        let mut P: i64;

        var1 = (self.t_fine - 128000) as i64;
        var2 = var1 * var1 * P6 as i64;
        var2 = var2 + ((var1 * P5 as i64) << 17);
        var2 = var2 + ((P4 as i64) << 35);
        var1 = ((var1 * var1 * P3 as i64) >> 8) + ((var1 * P2 as i64) << 12);
        var1 = ((((1 as i64) << 47) + var1) * P1 as i64) >> 33;
        if var1 == 0 {
            return Ok(0.0);
        }

        P = 1048576 - preRaw as i64;
        P = (((P << 31) - var2) * 3125) / var1;
        var1 = ((P9 as i64) * (P >> 13)) >> 25;
        var2 = ((P8 as i64) * P) >> 19;
        P = ((P + var1 + var2) >> 8) + ((P7 as i64) << 4);
        Ok((P / 256 / 100) as f32)
    }
    fn ReadByte(register: u8, i2c: &mut I2c) -> Result<u8, String> {
        let mut read_buf: [u8; 1] = [ 0x00 ];
        if let Err(e) = i2c.write_read(&[ register ], &mut read_buf) {
            return Err(e.to_string());
        };
        Ok(read_buf[0])
    }
    fn ReadUint16(register: u8, i2c: &mut I2c) -> Result<u16, String> {
        let mut read_buf = [ 0x00, 0x00 ];
        if let Err(e) = i2c.write_read(&[ register ], &mut read_buf) {
            return Err(e.to_string());
        };
        let h: u16 = (read_buf[1] << 8) as u16 ;
        let l: u16 = read_buf[0] as u16;
        let read_data = h + l;
        Ok(read_data)
    }
}


pub fn sensor() {
    let mut sensor = TempSensor {
        i2c_addr: 0x76,
        t_fine: i32::MIN, // データ校正用変数
    };
    let mut i2c = I2c::new().unwrap();
    
    // 初期化
    let _ = sensor.init(&mut i2c);
    std::thread::sleep(std::time::Duration::from_secs(1));

    // 処理
    let tmp = match sensor.ReadTmpAsync(&mut i2c) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            -999.9
        }
    };
    let pre = match sensor.ReadPreAsync(&mut i2c) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            -999.9
        }
    };
    let hum = match sensor.ReadHumAsync(&mut i2c) {
        Ok(v) => v,
        Err(e) => {
            println!("{}", e);
            999
        }
    };
    println!("tmp ({}) / pre ({}) / hum ({})", tmp, pre, hum);
}








